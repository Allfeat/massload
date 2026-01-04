//! # Allfeat Services
//!
//! Backend services for Allfeat apps - AI transformation, CSV parsing, caching.
//!
//! This crate provides:
//! - AI-powered CSV to MIDDS transformation using Claude
//! - CSV parsing with auto-encoding detection
//! - DSL-based field transformation
//! - Template caching for reuse
//!
//! ## Modules
//!
//! - [`ai`] - Claude AI integration for matrix generation
//! - [`parser`] - CSV parsing with encoding/delimiter detection
//! - [`transform`] - DSL transformation and grouping
//! - [`cache`] - Template registry for caching transformations

pub mod ai;
pub mod cache;
pub mod parser;
pub mod transform;

// Re-exports - AI
pub use ai::{generate_matrix, AiClient, AiError};

// Re-exports - Cache
pub use cache::{MatrixRegistry, StoredMatrix};

// Re-exports - Parser
pub use parser::{
    csv_to_json, decode_content, detect_delimiter, detect_encoding, parse_bytes_auto,
    parse_csv, parse_csv_file, parse_csv_file_auto, CsvError, ParseResult,
};

// Re-exports - Transform
pub use transform::dsl::{
    example_matrix, execute, execute_hashmap, operations_description, FieldTransform, Operation,
    SkippedRow, TransformError, TransformResult, TransformationMatrix,
};
pub use transform::flat_to_grouped;
pub use transform::pipeline::{
    set_pipeline_logger, transform_bytes, transform_csv, transform_records, 
    transform_with_matrix, CsvInfo, LogCallback, LogLevel, PipelineError, PipelineResult, 
    TransformOptions, TransformWithMatrixResult,
};

