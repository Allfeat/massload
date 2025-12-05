//! DSL Executor
//! 
//! Executes transformation matrices on CSV data to produce MIDDS flat records.

use serde_json::{Map, Value};
use std::collections::HashMap;

use super::matrix::{ExpandConfig, FieldTransform, TransformationMatrix};

/// Result of executing a transformation
#[derive(Debug)]
pub struct TransformResult {
    /// Successfully transformed records
    pub records: Vec<Value>,
    /// Errors encountered (row index, field, error message)
    pub errors: Vec<TransformError>,
    /// Rows skipped due to missing required fields
    pub skipped: Vec<SkippedRow>,
}

/// An error during transformation
#[derive(Debug, Clone)]
pub struct TransformError {
    pub row: usize,
    pub field: String,
    pub message: String,
}

/// A row that was skipped
#[derive(Debug, Clone)]
pub struct SkippedRow {
    pub row: usize,
    pub reason: String,
    pub missing_fields: Vec<String>,
}

impl TransformResult {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            errors: Vec::new(),
            skipped: Vec::new(),
        }
    }

    /// Check if transformation completed without errors
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get summary statistics
    pub fn summary(&self) -> String {
        format!(
            "Transformed: {} records, {} errors, {} skipped",
            self.records.len(),
            self.errors.len(),
            self.skipped.len()
        )
    }
}

impl Default for TransformResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Execute a transformation matrix on CSV data
/// 
/// # Arguments
/// * `csv_data` - Vector of JSON objects from CSV parsing (each object is a row)
/// * `matrix` - The transformation matrix to apply
/// 
/// # Returns
/// A TransformResult containing the transformed records and any errors
pub fn execute(csv_data: &[Value], matrix: &TransformationMatrix) -> TransformResult {
    let mut result = TransformResult::new();

    for (row_idx, row) in csv_data.iter().enumerate() {
        // Check if we need to expand this row into multiple records
        let expanded_rows = expand_row(row, matrix, row_idx);
        
        for (expanded_row, variant_overrides) in expanded_rows {
            match transform_row_with_overrides(&expanded_row, matrix, row_idx, variant_overrides.as_ref()) {
                Ok(Some(record)) => result.records.push(record),
                Ok(None) => {
                    // Row was intentionally skipped (e.g., missing required fields)
                }
                Err(skip) => result.skipped.push(skip),
            }
        }
    }

    result
}

/// Expand a row based on the matrix's expand configuration
fn expand_row(
    row: &Value,
    matrix: &TransformationMatrix,
    _row_idx: usize,
) -> Vec<(Value, Option<HashMap<String, FieldTransform>>)> {
    let row_obj = match row.as_object() {
        Some(obj) => obj,
        None => return vec![(row.clone(), None)],
    };

    match &matrix.expand {
        None => vec![(row.clone(), None)],
        
        Some(ExpandConfig::SplitRole { source, separator, mapping }) => {
            let role_value = row_obj.get(source)
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            if role_value.is_empty() || separator.is_empty() {
                return vec![(row.clone(), None)];
            }
            
            // Split the role value
            let roles: Vec<&str> = role_value.split(separator).map(|s| s.trim()).collect();
            
            if roles.len() <= 1 {
                return vec![(row.clone(), None)];
            }
            
            // Create one expanded row per role
            roles.iter().map(|role| {
                let mapped_role = mapping.get(*role).cloned()
                    .unwrap_or_else(|| role.to_string());
                
                // Create a modified row with the single role
                let mut new_row = row_obj.clone();
                new_row.insert(source.clone(), Value::String(mapped_role.clone()));
                
                // Create override for creatorRole to use the expanded value
                let mut overrides = HashMap::new();
                overrides.insert(
                    "creatorRole".to_string(),
                    FieldTransform::from_constant(Value::String(mapped_role)),
                );
                
                (Value::Object(new_row), Some(overrides))
            }).collect()
        }
        
        Some(ExpandConfig::MultipleColumns { variants }) => {
            let mut expanded = Vec::new();
            
            for variant in variants {
                // Check condition
                if let Some(ref cond_col) = variant.condition_column {
                    let cond_value = row_obj.get(cond_col)
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    
                    if cond_value.trim().is_empty() {
                        continue;
                    }
                }
                
                // This variant should produce a record
                expanded.push((row.clone(), Some(variant.overrides.clone())));
            }
            
            if expanded.is_empty() {
                vec![(row.clone(), None)]
            } else {
                expanded
            }
        }
    }
}

/// Execute transformation on CSV data provided as HashMaps
pub fn execute_hashmap(
    csv_data: &[HashMap<String, String>],
    matrix: &TransformationMatrix,
) -> TransformResult {
    let json_data: Vec<Value> = csv_data
        .iter()
        .map(|row| {
            let map: Map<String, Value> = row
                .iter()
                .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                .collect();
            Value::Object(map)
        })
        .collect();

    execute(&json_data, matrix)
}

/// Transform a single row with optional overrides from expansion
fn transform_row_with_overrides(
    row: &Value,
    matrix: &TransformationMatrix,
    row_idx: usize,
    overrides: Option<&HashMap<String, FieldTransform>>,
) -> Result<Option<Value>, SkippedRow> {
    let row_obj = match row.as_object() {
        Some(obj) => obj,
        None => {
            return Err(SkippedRow {
                row: row_idx,
                reason: "Row is not a JSON object".to_string(),
                missing_fields: Vec::new(),
            });
        }
    };

    let mut output = Map::new();
    let mut missing_required = Vec::new();

    for (target_field, transform) in &matrix.transforms {
        // Check if there's an override for this field
        let effective_transform = overrides
            .and_then(|o| o.get(target_field))
            .unwrap_or(transform);
        
        let value = apply_transform(row_obj, effective_transform);

        match value {
            Some(v) if !is_empty(&v) => {
                output.insert(target_field.clone(), v);
            }
            _ => {
                if effective_transform.required {
                    missing_required.push(target_field.clone());
                } else if let Some(default) = &effective_transform.default {
                    output.insert(target_field.clone(), default.clone());
                }
            }
        }
    }

    if !missing_required.is_empty() {
        return Err(SkippedRow {
            row: row_idx,
            reason: "Missing required fields".to_string(),
            missing_fields: missing_required,
        });
    }

    // Skip empty rows
    if output.is_empty() {
        return Ok(None);
    }

    Ok(Some(Value::Object(output)))
}

/// Apply a field transformation
fn apply_transform(row: &Map<String, Value>, transform: &FieldTransform) -> Option<Value> {
    // Get initial value from source column(s) or constant
    let mut value = if let Some(source) = &transform.source {
        // Single source
        row.get(source).cloned()
    } else if let Some(sources) = &transform.sources {
        // Multiple sources - concatenate them
        let parts: Vec<String> = sources.iter()
            .filter_map(|s| row.get(s))
            .filter_map(|v| v.as_str())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        
        if parts.is_empty() {
            None
        } else {
            Some(Value::String(parts.join(&transform.concat_separator)))
        }
    } else {
        transform.constant.clone()
    };

    // If no value and we have a default, use it
    if value.is_none() || is_empty(value.as_ref().unwrap()) {
        if let Some(default) = &transform.default {
            value = Some(default.clone());
        }
    }

    // Apply operations in sequence
    if let Some(mut v) = value {
        for op in &transform.operations {
            v = op.apply(&v);
        }

        // If result is empty after operations, try default again
        if is_empty(&v) {
            if let Some(default) = &transform.default {
                return Some(default.clone());
            }
            return None;
        }

        return Some(v);
    }

    None
}

/// Check if a value is "empty" (null, empty string, etc.)
fn is_empty(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) => s.trim().is_empty(),
        Value::Array(a) => a.is_empty(),
        Value::Object(o) => o.is_empty(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::dsl::matrix::example_matrix;

    #[test]
    fn test_execute_simple() {
        let csv_data = vec![
            serde_json::json!({
                "Code ISWC": "T-123.456.789-0",
                "Titre": "  Ma Chanson  ",
                "Role": "CA",
                "IPI": "123456789",
                "Instrumental": "oui"
            }),
            serde_json::json!({
                "Code ISWC": "T9876543210",
                "Titre": "Another Song",
                "Role": "A",
                "IPI": "987654321",
                "Instrumental": "non"
            }),
        ];

        let matrix = example_matrix();
        let result = execute(&csv_data, &matrix);

        assert!(result.is_ok());
        assert_eq!(result.records.len(), 2);

        // Check first record
        let first = &result.records[0];
        assert_eq!(first["iswc"], "T1234567890");
        assert_eq!(first["title"], "Ma Chanson");
        assert_eq!(first["creatorRole"], "Composer");
        assert_eq!(first["creatorIpi"], 123456789);
        assert_eq!(first["instrumental"], true);

        // Check second record
        let second = &result.records[1];
        assert_eq!(second["creatorRole"], "Author");
        assert_eq!(second["instrumental"], false);
    }

    #[test]
    fn test_missing_required_field() {
        let csv_data = vec![serde_json::json!({
            "Titre": "Missing ISWC",
            "Role": "CA"
        })];

        let matrix = example_matrix();
        let result = execute(&csv_data, &matrix);

        assert_eq!(result.records.len(), 0);
        assert_eq!(result.skipped.len(), 1);
        assert!(result.skipped[0].missing_fields.contains(&"iswc".to_string()));
    }

    #[test]
    fn test_constant_value() {
        let mut matrix = TransformationMatrix::new();
        matrix.transforms.insert(
            "language".to_string(),
            super::super::matrix::FieldTransform::from_constant(Value::String("French".to_string())),
        );

        let csv_data = vec![serde_json::json!({
            "any_field": "any_value"
        })];

        let result = execute(&csv_data, &matrix);
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.records[0]["language"], "French");
    }

    #[test]
    fn test_default_value() {
        let mut matrix = TransformationMatrix::new();
        matrix.transforms.insert(
            "instrumental".to_string(),
            FieldTransform::from_source("Missing Column")
                .with_default(Value::Bool(false)),
        );

        let csv_data = vec![serde_json::json!({
            "other_field": "value"
        })];

        let result = execute(&csv_data, &matrix);
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.records[0]["instrumental"], false);
    }

    #[test]
    fn test_multiple_sources_concat() {
        let mut matrix = TransformationMatrix::new();
        matrix.transforms.insert(
            "title".to_string(),
            FieldTransform::from_sources(
                vec!["Title Prefix".to_string(), "Title Main".to_string()],
                " "
            ),
        );

        let csv_data = vec![serde_json::json!({
            "Title Prefix": "The Amazing",
            "Title Main": "Journey"
        })];

        let result = execute(&csv_data, &matrix);
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.records[0]["title"], "The Amazing Journey");
    }

    #[test]
    fn test_multiple_sources_skip_empty() {
        let mut matrix = TransformationMatrix::new();
        matrix.transforms.insert(
            "title".to_string(),
            FieldTransform::from_sources(
                vec!["Title Prefix".to_string(), "Title Main".to_string()],
                " "
            ),
        );

        let csv_data = vec![serde_json::json!({
            "Title Prefix": "",
            "Title Main": "Solo Title"
        })];

        let result = execute(&csv_data, &matrix);
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.records[0]["title"], "Solo Title");
    }
}

