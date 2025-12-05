//! # Massload - MIDDS Musical Work validation and transformation
//!
//! Massload transforms CSV files from various music industry sources (SACEM, ASCAP, GEMA, etc.)
//! into MIDDS format for blockchain registration.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
//! │   CSV File  │────▶│   Parser    │────▶│  Transform  │────▶│  MIDDS JSON │
//! │  (ISO/UTF8) │     │  (auto-enc) │     │  (AI + DSL) │     │  (grouped)  │
//! └─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
//! ```
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use massload::{transform_csv, TransformOptions};
//!
//! #[tokio::main]
//! async fn main() {
//!     let result = transform_csv("input.csv", TransformOptions::default()).await.unwrap();
//!     println!("Transformed {} works", result.grouped.len());
//! }
//! ```
//!
//! ## Modules
//!
//! - [`error`] - Hierarchical error types
//! - [`models`] - Domain models (GroupedWork, Creator, PartyId)
//! - [`parser`] - CSV parsing with auto-detection
//! - [`transform`] - DSL, grouping, and pipeline
//! - [`validation`] - MIDDS schema validation
//! - [`cache`] - Template caching
//! - [`ai`] - AI-powered matrix generation
//! - [`api`] - HTTP API server

// Core modules
pub mod error;
pub mod models;

// Parsing
pub mod parser;

// Transformation
pub mod transform;

// Validation
pub mod validation;

// Caching
pub mod cache;

// AI
pub mod ai;

// HTTP API
pub mod api;

// =============================================================================
// Re-exports - Error types
// =============================================================================

pub use error::{
    CsvError as CsvErrorNew,
    TransformError as TransformErrorNew,
    AiError as AiErrorNew,
    RegistryError,
    ValidationError as ValidationErrorNew,
    PipelineError as PipelineErrorNew,
    ServerError,
};

// =============================================================================
// Re-exports - Models
// =============================================================================

pub use models::{
    PartyId,
    CreatorRole,
    Creator,
    MusicalWorkType,
    GroupedWork,
};

// =============================================================================
// Re-exports - Validation
// =============================================================================

pub use validation::{
    is_valid, 
    validate, 
    is_valid_musical_work_grouped, 
    validate_musical_work_grouped,
    is_valid_musical_work_flat,
    validate_musical_work_flat,
};

// =============================================================================
// Re-exports - Grouper
// =============================================================================

pub use transform::flat_to_grouped;

// =============================================================================
// Re-exports - CSV Parsing
// =============================================================================

pub use parser::{
    csv_to_json, 
    parse_csv, 
    parse_csv_file, 
    parse_csv_file_auto,
    parse_bytes_auto,
    detect_encoding,
    detect_delimiter,
    decode_content,
    CsvError,
    ParseResult,
};

// =============================================================================
// Re-exports - DSL
// =============================================================================

pub use transform::dsl::{
    TransformationMatrix,
    FieldTransform,
    Operation,
    execute,
    execute_hashmap,
    TransformResult,
    TransformError,
    SkippedRow,
    operations_description,
    example_matrix,
};

// =============================================================================
// Re-exports - AI Client
// =============================================================================

pub use ai::{AiClient, AiError, generate_matrix};

// =============================================================================
// Re-exports - Registry (Cache)
// =============================================================================

pub use cache::{MatrixRegistry, StoredMatrix};

// =============================================================================
// Re-exports - Pipeline
// =============================================================================

pub use transform::pipeline::{
    transform_csv,
    transform_bytes,
    transform_records,
    transform_with_matrix,
    TransformOptions,
    PipelineResult,
    PipelineError,
    CsvInfo,
    TransformWithMatrixResult,
};

// =============================================================================
// Re-exports - API
// =============================================================================

pub use api::types::{
    UploadResponse,
    ResponseMetadata,
    CsvMetadata,
    ValidationStats,
    ValidationError,
    error_response,
};

// Server
pub mod server {
    pub use crate::api::server::start_server;
}
