//! REST API types for frontend integration.
//!
//! Returns MIDDS format directly - no conversion needed in frontend.
//! Format matches allfeat-sdk/midds-v2 exactly.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::transform::pipeline::PipelineResult;

/// Response sent to frontend after CSV upload and transformation.
/// `musical_works` contains MIDDS format ready for blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    /// Unique job identifier
    pub job_id: String,
    
    /// Status: "ready", "warning", "error"
    pub status: String,
    
    /// Musical works in MIDDS format - ready for blockchain
    pub musical_works: Vec<Value>,
    
    /// Metadata about the transformation
    pub metadata: ResponseMetadata,
}

/// Metadata about the transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    /// Total number of works
    pub total_works: usize,
    
    /// Estimated cost in AFT
    pub estimated_cost: String,
    
    /// Template ID used (if cached)
    pub matrix_id: Option<String>,
    
    /// Whether a cached template was used
    pub cached: bool,
    
    /// CSV info
    pub csv_info: CsvMetadata,
    
    /// Validation stats
    pub validation: ValidationStats,
}

/// CSV file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvMetadata {
    pub encoding: String,
    pub delimiter: String,
    pub row_count: usize,
    pub columns: Vec<String>,
}

/// Validation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationStats {
    pub valid: usize,
    pub invalid: usize,
    pub errors: Vec<ValidationError>,
}

/// A validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    pub record_index: usize,
    pub errors: Vec<String>,
}

/// Convert PipelineResult to UploadResponse
/// The grouped works are already in MIDDS format!
impl From<PipelineResult> for UploadResponse {
    fn from(result: PipelineResult) -> Self {
        // grouped is already in MIDDS format from grouper.rs
        // Just need to ensure camelCase naming
        let musical_works: Vec<Value> = result
            .grouped
            .into_iter()
            .map(ensure_midds_format)
            .collect();

        let total = musical_works.len();
        let cost_per_work = 0.05;
        let estimated_cost = format!("{:.2} AFT", total as f64 * cost_per_work);

        UploadResponse {
            job_id: Uuid::new_v4().to_string(),
            status: if result.invalid_count == 0 { "ready" } else { "warning" }.to_string(),
            musical_works,
            metadata: ResponseMetadata {
                total_works: total,
                estimated_cost,
                cached: result.template_id.is_some(),
                matrix_id: result.template_id,
                csv_info: CsvMetadata {
                    encoding: result.csv_info.encoding,
                    delimiter: result.csv_info.delimiter.to_string(),
                    row_count: result.csv_info.row_count,
                    columns: result.csv_info.headers,
                },
                validation: ValidationStats {
                    valid: result.valid_count,
                    invalid: result.invalid_count,
                    errors: result.validation_errors.into_iter()
                        .map(|(idx, errs)| ValidationError {
                            record_index: idx,
                            errors: errs,
                        })
                        .collect(),
                },
            },
        }
    }
}

/// Ensure the work is in exact MIDDS format for blockchain
/// grouper.rs already produces this format, just ensure consistency
fn ensure_midds_format(work: Value) -> Value {
    // The grouped format from grouper.rs is already MIDDS-compatible:
    // {
    //   "iswc": "T1234567890",
    //   "title": "My Song",
    //   "creationYear": 2024,
    //   "instrumental": false,
    //   "language": "English",
    //   "bpm": 120,
    //   "key": "Am",
    //   "workType": "Original",
    //   "creators": [{ "id": { "Ipi": 123 }, "role": "Composer" }],
    //   "classicalInfo": { "opus": "Op. 1", ... }
    // }
    //
    // This matches the MIDDS SDK format exactly!
    work
}

/// Create an error response
pub fn error_response(error: &str) -> Value {
    json!({
        "jobId": Uuid::new_v4().to_string(),
        "status": "error",
        "error": error,
        "musicalWorks": [],
        "metadata": {
            "totalWorks": 0,
            "estimatedCost": "0 AFT",
            "matrixId": null,
            "cached": false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midds_format_passthrough() {
        // The grouped format is already MIDDS
        let work = json!({
            "iswc": "T1234567890",
            "title": "My Song",
            "creationYear": 2024,
            "instrumental": false,
            "language": "English",
            "creators": [
                { "id": { "Ipi": 123456789 }, "role": "Composer" }
            ],
            "workType": "Original"
        });

        let midds = ensure_midds_format(work.clone());
        
        assert_eq!(midds["iswc"], "T1234567890");
        assert_eq!(midds["title"], "My Song");
        assert_eq!(midds["creationYear"], 2024);
        assert_eq!(midds["creators"][0]["role"], "Composer");
    }
}

