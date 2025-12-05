//! High-level pipeline API for CSV to MIDDS transformation.
//!
//! This module provides easy-to-use functions that combine all steps:
//! parsing, matrix generation, transformation, validation, and grouping.
//!
//! # Example
//!
//! ```rust,ignore
//! use massload::pipeline::{transform_csv, TransformOptions};
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let result = transform_csv(
//!         Path::new("catalog.csv"),
//!         TransformOptions::default(),
//!     ).await?;
//!
//!     println!("Transformed {} works", result.grouped.len());
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use thiserror::Error;

use crate::parser::{parse_csv_file_auto, parse_bytes_auto, CsvError, ParseResult};
use crate::transform::dsl::{execute, TransformationMatrix};
use super::grouper::flat_to_grouped;
use crate::api::logs::{log_info, log_success, log_warning, log_error};
use crate::cache::MatrixRegistry;
use crate::validation::{validate_musical_work_flat, validate_musical_work_grouped};
use crate::ai::{AiClient, AiError};

/// Pipeline errors
#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("CSV parsing error: {0}")]
    CsvError(#[from] CsvError),

    #[error("AI error: {0}")]
    AiError(#[from] AiError),

    #[error("Matrix error: {0}")]
    MatrixError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Options for the transformation pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformOptions {
    /// Use a specific matrix file instead of AI/cache
    pub matrix_path: Option<String>,

    /// Number of rows to send to AI for analysis
    pub preview_rows: usize,

    /// Skip validation step
    pub skip_validation: bool,

    /// Don't use cached templates
    pub no_cache: bool,

    /// Don't save generated matrix to cache
    pub no_save: bool,
}

impl Default for TransformOptions {
    fn default() -> Self {
        Self {
            matrix_path: None,
            preview_rows: 10,
            skip_validation: false,
            no_cache: false,
            no_save: false,
        }
    }
}

/// Result of a complete transformation pipeline
#[derive(Debug, Clone, Serialize)]
pub struct PipelineResult {
    /// Flat records (one per creator)
    pub flat: Vec<Value>,

    /// Grouped records (one per work, with creators array)
    pub grouped: Vec<Value>,

    /// Number of valid records
    pub valid_count: usize,

    /// Number of invalid records
    pub invalid_count: usize,

    /// Validation errors (record index, errors)
    pub validation_errors: Vec<(usize, Vec<String>)>,

    /// Matrix used for transformation
    pub matrix: TransformationMatrix,

    /// Template ID if a cached template was used or created
    pub template_id: Option<String>,

    /// CSV parsing metadata
    pub csv_info: CsvInfo,
}

/// CSV file information
#[derive(Debug, Clone, Serialize)]
pub struct CsvInfo {
    pub encoding: String,
    pub delimiter: char,
    pub headers: Vec<String>,
    pub row_count: usize,
}

/// Transform a CSV file to MIDDS format.
///
/// This is the main entry point for the pipeline. It:
/// 1. Parses the CSV with auto-detection
/// 2. Finds or generates a transformation matrix
/// 3. Executes the transformation
/// 4. Validates the results
/// 5. Groups records by ISWC
///
/// # Arguments
/// * `path` - Path to the CSV file
/// * `options` - Transformation options
///
/// # Returns
/// A `PipelineResult` containing flat records, grouped records, and metadata
pub async fn transform_csv(
    path: &Path,
    options: TransformOptions,
) -> Result<PipelineResult, PipelineError> {
    // 1. Parse CSV
    let parse_result = parse_csv_file_auto(path)?;
    transform_parsed(parse_result, options, Some(path)).await
}

/// Transform CSV bytes to MIDDS format.
///
/// Same as `transform_csv` but accepts raw bytes instead of a file path.
pub async fn transform_bytes(
    bytes: &[u8],
    options: TransformOptions,
) -> Result<PipelineResult, PipelineError> {
    let parse_result = parse_bytes_auto(bytes)?;
    transform_parsed(parse_result, options, None).await
}

/// Transform already-parsed CSV data.
///
/// Useful when you've already parsed the CSV and want to transform it.
pub async fn transform_records(
    records: Vec<Value>,
    headers: Vec<String>,
    options: TransformOptions,
) -> Result<PipelineResult, PipelineError> {
    let parse_result = ParseResult {
        records,
        encoding: "utf-8".to_string(),
        delimiter: ',',
        headers,
    };
    transform_parsed(parse_result, options, None).await
}

/// Internal: transform parsed CSV data
async fn transform_parsed(
    parse_result: ParseResult,
    options: TransformOptions,
    source_path: Option<&Path>,
) -> Result<PipelineResult, PipelineError> {
    // Step 1: CSV Info
    log_info("📖 Reading CSV file...");
    log_info("Detecting encoding and separator...");
    log_success(format!("Detected encoding: {}", parse_result.encoding));
    log_success(format!("Detected separator: '{}'", format_delimiter(parse_result.delimiter)));
    log_success(format!("Read {} rows", parse_result.records.len()));
    
    let csv_info = CsvInfo {
        encoding: parse_result.encoding.clone(),
        delimiter: parse_result.delimiter,
        headers: parse_result.headers.clone(),
        row_count: parse_result.records.len(),
    };

    if parse_result.records.is_empty() {
        return Err(PipelineError::MatrixError("CSV file is empty".to_string()));
    }

    // Display columns
    log_info(format!("📋 CSV has {} columns:", parse_result.headers.len()));
    for (i, col) in parse_result.headers.iter().enumerate() {
        log_info(format!("[{:2}] {}", i + 1, col));
    }

    // Step 2: Get or generate matrix (with fallback)
    log_info("🔄 Auto-detecting format and transforming...");
    let (matrix, template_id, transform_result, valid_count, invalid_count, validation_errors) = 
        get_matrix_with_fallback(&parse_result, &options, source_path).await?;

    // Step 5: Group by ISWC
    log_info("📦 Grouping by ISWC...");
    let grouped = flat_to_grouped(transform_result.records.clone());
    log_success(format!("{} musical works", grouped.len()));

    // Step 6: Validate grouped format against schema (before sending to blockchain)
    if !options.skip_validation {
        log_info("✔️  Validating grouped MIDDS format...");
        let mut grouped_errors = 0;
        for (i, work) in grouped.iter().enumerate() {
            if let Err(errs) = validate_musical_work_grouped(work) {
                grouped_errors += 1;
                if grouped_errors <= 3 {
                    log_error(format!("Work {}: {}", i, errs.join(", ")));
                }
            }
        }
        if grouped_errors > 0 {
            log_warning(format!("{} works failed grouped validation", grouped_errors));
        } else {
            log_success("All grouped works valid for blockchain!");
        }
    }

    Ok(PipelineResult {
        flat: transform_result.records,
        grouped,
        valid_count,
        invalid_count,
        validation_errors,
        matrix,
        template_id,
        csv_info,
    })
}

/// Format delimiter for display
fn format_delimiter(d: char) -> &'static str {
    match d {
        ';' => ";",
        ',' => ",",
        '\t' => "TAB",
        '|' => "|",
        _ => "?",
    }
}

/// Get matrix and execute transformation with fallback to AI if all cached templates fail
/// 
/// Algorithm (like massdrop's SmartTransformer):
/// 1. Find ALL compatible cached templates
/// 2. Try each one (sorted by success rate)
/// 3. Stop at first one that produces valid results
/// 4. If ALL fail → fallback to AI
async fn get_matrix_with_fallback(
    parse_result: &ParseResult,
    options: &TransformOptions,
    source_path: Option<&Path>,
) -> Result<(TransformationMatrix, Option<String>, super::dsl::TransformResult, usize, usize, Vec<(usize, Vec<String>)>), PipelineError> {
    
    // Option 1: Use provided matrix file (no fallback)
    if let Some(ref matrix_path) = options.matrix_path {
        log_info(format!("Using provided matrix file: {}", matrix_path));
        let content = std::fs::read_to_string(matrix_path)?;
        let matrix = TransformationMatrix::from_json(&content)
            .map_err(|e| PipelineError::MatrixError(e.to_string()))?;
        return try_matrix(parse_result, matrix, None, options);
    }

    // Option 2: Try ALL compatible cached templates (sorted by success rate)
    if !options.no_cache {
        log_info("Looking for compatible cached templates...");
        let registry = MatrixRegistry::new();
        let compatible = registry.find_compatible(&parse_result.headers);
        
        if compatible.is_empty() {
            log_warning("No compatible templates found");
        } else {
            log_success(format!("Found {} compatible template(s)", compatible.len()));
            
            // Try each template until one works
            for (i, (template, score)) in compatible.iter().enumerate() {
                log_info(format!("→ Trying template {}/{}: {} (score: {:.0}%, success rate: {:.0}%)", 
                    i + 1, compatible.len(), template.name, score * 100.0, template.success_rate * 100.0));
                
                let result = try_matrix(parse_result, template.matrix.clone(), Some(template.id.clone()), options);
                
                if let Ok((ref _m, ref _tid, ref tr, valid, _invalid, ref _errs)) = result {
                    // Update stats
                    let mut registry_mut = MatrixRegistry::new();
                    let success = valid > 0;
                    registry_mut.update_stats(&template.id, success);
                    
                    if success {
                        log_success(format!("✅ Template \"{}\" worked!", template.name));
                        return result;
                    } else {
                        log_warning(format!("Template \"{}\" failed ({} records, 0 valid)", template.name, tr.records.len()));
                    }
                }
            }
            
            log_warning(format!("All {} cached templates failed", compatible.len()));
        }
    }

    // Option 3: Fallback to AI
    log_info("🤖 Fallback: Generating new matrix with AI...");
    log_info("Using Claude API...");
    let client = AiClient::from_env()?;
    let preview_count = options.preview_rows.min(parse_result.records.len());
    let preview = &parse_result.records[..preview_count];
    log_info(format!("Sending {} preview rows + unique values from {} total rows to AI...", preview_count, parse_result.records.len()));
    let matrix = client.generate_matrix_full(preview, &parse_result.records).await?;
    log_success("AI matrix generated successfully");
    log_info(format!("Fields mapped: {}", matrix.transforms.len()));
    
    // Save AI matrix to cache
    let template_id = if !options.no_save {
        let mut registry = MatrixRegistry::new();
        let name = source_path
            .and_then(|p| p.file_stem())
            .and_then(|s| s.to_str())
            .unwrap_or("auto-generated");
        registry.save(matrix.clone(), name, parse_result.headers.clone()).ok()
    } else {
        None
    };
    
    let result = try_matrix(parse_result, matrix, template_id.clone(), options);
    
    // Update AI template stats
    if let (Some(ref tid), Ok((_, _, _, valid, _, _))) = (&template_id, &result) {
        let mut registry = MatrixRegistry::new();
        registry.update_stats(tid, *valid > 0);
        log_success(format!("→ Saved as: {}", tid));
    }
    
    result
}

/// Try a matrix and return results
fn try_matrix(
    parse_result: &ParseResult,
    matrix: TransformationMatrix,
    template_id: Option<String>,
    options: &TransformOptions,
) -> Result<(TransformationMatrix, Option<String>, super::dsl::TransformResult, usize, usize, Vec<(usize, Vec<String>)>), PipelineError> {
    print_matrix_mapping(&matrix);
    
    log_info("⚙️  Executing transformation...");
    let transform_result = execute(&parse_result.records, &matrix);
    print_transform_result(&transform_result);
    
    log_info("✔️  Validating records...");
    let (valid_count, invalid_count, validation_errors) = if options.skip_validation {
        log_info("(validation skipped)");
        (transform_result.records.len(), 0, vec![])
    } else {
        let result = validate_records(&transform_result.records);
        print_validation_result(&result);
        result
    };
    
    Ok((matrix, template_id, transform_result, valid_count, invalid_count, validation_errors))
}

/// Print matrix mapping
fn print_matrix_mapping(matrix: &TransformationMatrix) {
    log_info("🗺️  Matrix mapping:");
    for (field, transform) in &matrix.transforms {
        if let Some(ref src) = transform.source {
            log_info(format!("{} → {}", src, field));
        } else if let Some(ref srcs) = transform.sources {
            log_info(format!("[{}] → {}", srcs.join(" + "), field));
        } else if transform.constant.is_some() {
            log_info(format!("(constant) → {}", field));
        }
    }
}

/// Print transformation result details
fn print_transform_result(result: &super::dsl::TransformResult) {
    log_success(format!("Generated {} flat records", result.records.len()));
    
    if !result.errors.is_empty() {
        log_warning(format!("{} errors during transformation", result.errors.len()));
        for err in result.errors.iter().take(3) {
            log_error(format!("{:?}", err));
        }
    }
    if !result.skipped.is_empty() {
        log_warning(format!("{} rows skipped (missing required fields)", result.skipped.len()));
        
        // Group by reason
        let mut reasons: std::collections::HashMap<String, Vec<usize>> = std::collections::HashMap::new();
        for skip in &result.skipped {
            let key = if skip.missing_fields.is_empty() {
                skip.reason.clone()
            } else {
                format!("Missing: {}", skip.missing_fields.join(", "))
            };
            reasons.entry(key).or_default().push(skip.row);
        }
        
        for (reason, rows) in reasons.iter().take(5) {
            let row_sample: Vec<String> = rows.iter().take(5).map(|r| r.to_string()).collect();
            let more = if rows.len() > 5 { format!("... +{}", rows.len() - 5) } else { String::new() };
            log_warning(format!("• {} (rows: {}{})", reason, row_sample.join(", "), more));
        }
    }
}

/// Print validation result
fn print_validation_result(result: &(usize, usize, Vec<(usize, Vec<String>)>)) {
    if result.1 == 0 {
        log_success(format!("All {} records valid!", result.0));
    } else {
        log_success(format!("Valid: {}", result.0));
        log_error(format!("Invalid: {}", result.1));
    }
}

/// Validate records and return statistics
fn validate_records(records: &[Value]) -> (usize, usize, Vec<(usize, Vec<String>)>) {
    let mut valid = 0;
    let mut invalid = 0;
    let mut errors = Vec::new();

    for (i, record) in records.iter().enumerate() {
        match validate_musical_work_flat(record) {
            Ok(()) => valid += 1,
            Err(errs) => {
                invalid += 1;
                if errors.len() < 10 {
                    errors.push((i, errs));
                }
            }
        }
    }

    (valid, invalid, errors)
}

/// Transform with a specific matrix (no AI, no cache)
pub fn transform_with_matrix(
    records: &[Value],
    matrix: &TransformationMatrix,
    validate: bool,
) -> TransformWithMatrixResult {
    let result = execute(records, matrix);

    let (valid_count, invalid_count, validation_errors) = if validate {
        validate_records(&result.records)
    } else {
        (result.records.len(), 0, vec![])
    };

    let grouped = flat_to_grouped(result.records.clone());

    TransformWithMatrixResult {
        flat: result.records,
        grouped,
        valid_count,
        invalid_count,
        validation_errors,
        skipped: result.skipped.len(),
        errors: result.errors.len(),
    }
}

/// Result of transform_with_matrix
#[derive(Debug, Clone, Serialize)]
pub struct TransformWithMatrixResult {
    pub flat: Vec<Value>,
    pub grouped: Vec<Value>,
    pub valid_count: usize,
    pub invalid_count: usize,
    pub validation_errors: Vec<(usize, Vec<String>)>,
    pub skipped: usize,
    pub errors: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let opts = TransformOptions::default();
        assert_eq!(opts.preview_rows, 10);
        assert!(!opts.skip_validation);
        assert!(!opts.no_cache);
    }

    #[test]
    fn test_transform_with_matrix() {
        // Use column names that match the example_matrix
        let records = vec![
            serde_json::json!({
                "Code ISWC": "T-123.456.789-0",
                "Titre": "Test Song",
                "IPI": "123456789",
                "Role": "CA",
                "Instrumental": "non"
            })
        ];

        let matrix = crate::transform::dsl::example_matrix();
        let result = transform_with_matrix(&records, &matrix, false);

        assert_eq!(result.flat.len(), 1);
        assert_eq!(result.grouped.len(), 1);
        assert_eq!(result.flat[0]["iswc"], "T1234567890");
        assert_eq!(result.flat[0]["title"], "Test Song");
    }
}

