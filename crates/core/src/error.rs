//! Error types for the Massload transformation pipeline.
//!
//! This module defines a hierarchy of error types following best practices:
//!
//! - [`CsvError`] - CSV parsing errors
//! - [`TransformError`] - DSL transformation errors
//! - [`AiError`] - AI client errors
//! - [`RegistryError`] - Template registry errors
//! - [`PipelineError`] - Top-level orchestration errors
//!
//! Error conversion is automatic via `From` implementations,
//! allowing `?` to work across error boundaries.

use thiserror::Error;

// =============================================================================
// CSV Parsing Errors
// =============================================================================

/// Errors during CSV parsing.
#[derive(Debug, Error)]
pub enum CsvError {
    /// Failed to read file.
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    /// Failed to detect encoding.
    #[error("Failed to detect encoding: {0}")]
    EncodingError(String),

    /// Invalid CSV format.
    #[error("Invalid CSV format: {0}")]
    ParseError(String),

    /// Empty file.
    #[error("CSV file is empty")]
    EmptyFile,

    /// No headers found.
    #[error("No headers found in CSV")]
    NoHeaders,
}

// =============================================================================
// Transformation Errors
// =============================================================================

/// Errors during DSL transformation.
#[derive(Debug, Error)]
pub enum TransformError {
    /// Invalid transformation matrix.
    #[error("Invalid transformation matrix: {0}")]
    InvalidMatrix(String),

    /// Missing required source column.
    #[error("Missing source column: {0}")]
    MissingColumn(String),

    /// Operation execution failed.
    #[error("Operation failed on field '{field}': {message}")]
    OperationFailed { field: String, message: String },

    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

// =============================================================================
// AI Client Errors
// =============================================================================

/// Errors from the AI client.
#[derive(Debug, Error)]
pub enum AiError {
    /// Missing API key.
    #[error("Missing ANTHROPIC_API_KEY environment variable")]
    MissingApiKey,

    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    HttpError(String),

    /// Invalid response from AI.
    #[error("Invalid AI response: {0}")]
    InvalidResponse(String),

    /// Rate limited.
    #[error("Rate limited, retry after {0} seconds")]
    RateLimited(u64),

    /// Timeout.
    #[error("Request timed out")]
    Timeout,
}

// =============================================================================
// Registry Errors
// =============================================================================

/// Errors from the template registry.
#[derive(Debug, Error)]
pub enum RegistryError {
    /// Template not found.
    #[error("Template not found: {0}")]
    NotFound(String),

    /// Failed to save template.
    #[error("Failed to save template: {0}")]
    SaveError(String),

    /// Invalid template data.
    #[error("Invalid template: {0}")]
    InvalidTemplate(String),

    /// IO error.
    #[error("Registry IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON error.
    #[error("Registry JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

// =============================================================================
// Validation Errors
// =============================================================================

/// Errors during MIDDS validation.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Schema validation failed.
    #[error("Validation failed: {errors:?}")]
    SchemaError { errors: Vec<String> },

    /// Missing required field.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid field value.
    #[error("Invalid value for field '{field}': {message}")]
    InvalidValue { field: String, message: String },
}

// =============================================================================
// Pipeline Errors (top-level)
// =============================================================================

/// Top-level pipeline orchestration errors.
///
/// This is the main error type returned by [`crate::pipeline::transform_csv`].
/// It wraps all lower-level errors and adds pipeline-specific variants.
#[derive(Debug, Error)]
pub enum PipelineError {
    /// CSV parsing error.
    #[error("CSV error: {0}")]
    Csv(#[from] CsvError),

    /// Transformation error.
    #[error("Transform error: {0}")]
    Transform(#[from] TransformError),

    /// AI client error.
    #[error("AI error: {0}")]
    Ai(#[from] AiError),

    /// Registry error.
    #[error("Registry error: {0}")]
    Registry(#[from] RegistryError),

    /// Validation error.
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// No records to transform.
    #[error("No records to transform")]
    EmptyInput,

    /// All records were invalid.
    #[error("All {0} records failed validation")]
    AllInvalid(usize),
}

// =============================================================================
// Server Errors
// =============================================================================

/// HTTP server errors.
#[derive(Debug, Error)]
pub enum ServerError {
    /// Pipeline error.
    #[error("Pipeline error: {0}")]
    Pipeline(#[from] PipelineError),

    /// Invalid request.
    #[error("Invalid request: {0}")]
    BadRequest(String),

    /// Server internal error.
    #[error("Internal server error: {0}")]
    Internal(String),
}

// =============================================================================
// Result Type Aliases
// =============================================================================

/// Result type for CSV operations.
pub type CsvResult<T> = Result<T, CsvError>;

/// Result type for transformation operations.
pub type TransformResult<T> = Result<T, TransformError>;

/// Result type for AI operations.
pub type AiResult<T> = Result<T, AiError>;

/// Result type for registry operations.
pub type RegistryResult<T> = Result<T, RegistryError>;

/// Result type for pipeline operations.
pub type PipelineResult<T> = Result<T, PipelineError>;

/// Result type for server operations.
pub type ServerResult<T> = Result<T, ServerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion_chain() {
        // CsvError -> PipelineError
        let csv_err = CsvError::EmptyFile;
        let pipeline_err: PipelineError = csv_err.into();
        assert!(pipeline_err.to_string().contains("empty"));

        // TransformError -> PipelineError
        let transform_err = TransformError::MissingColumn("title".into());
        let pipeline_err: PipelineError = transform_err.into();
        assert!(pipeline_err.to_string().contains("title"));
    }

    #[test]
    fn test_validation_error_format() {
        let err = ValidationError::InvalidValue {
            field: "iswc".into(),
            message: "must start with T".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("iswc"));
        assert!(msg.contains("must start with T"));
    }
}

