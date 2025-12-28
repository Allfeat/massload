//! # Allfeat Hub Backend
//!
//! Unified backend server for the Allfeat Apps Hub.
//!
//! This crate provides the HTTP API server that powers the Allfeat Apps Hub,
//! combining functionality from Massload, Register, and Protect.
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
//! ## Modules
//!
//! - [`api`] - HTTP API server (routes, handlers, SSE logs)
//!
//! ## External Crates
//!
//! Core functionality is provided by:
//! - `allfeat-core` - Domain models, validation, error types
//! - `allfeat-services` - AI transformation, CSV parsing, caching

// HTTP API (server-specific)
pub mod api;

// =============================================================================
// Re-exports from allfeat-core
// =============================================================================

pub use allfeat_core::{
    // Models
    Creator, CreatorRole, GroupedWork, MusicalWorkType, PartyId,
    // Validation
    is_valid, is_valid_musical_work_flat, is_valid_musical_work_grouped, validate,
    validate_musical_work_flat, validate_musical_work_grouped,
    // Errors
    AiError as AiErrorCore, CsvError as CsvErrorCore, PipelineError as PipelineErrorCore,
    RegistryError, ServerError, TransformError as TransformErrorCore, ValidationError,
};

// =============================================================================
// Re-exports from allfeat-services
// =============================================================================

pub use allfeat_services::{
    // AI
    generate_matrix, AiClient, AiError,
    // Cache
    MatrixRegistry, StoredMatrix,
    // Parser
    csv_to_json, decode_content, detect_delimiter, detect_encoding, parse_bytes_auto,
    parse_csv, parse_csv_file, parse_csv_file_auto, CsvError, ParseResult,
    // Transform DSL
    example_matrix, execute, execute_hashmap, operations_description, FieldTransform, Operation,
    SkippedRow, TransformError, TransformResult, TransformationMatrix,
    // Transform Pipeline
    flat_to_grouped, transform_bytes, transform_csv, transform_records, transform_with_matrix,
    CsvInfo, PipelineError, PipelineResult, TransformOptions, TransformWithMatrixResult,
};

// =============================================================================
// Re-exports - API
// =============================================================================

pub use api::types::{
    error_response, CsvMetadata, ResponseMetadata, UploadResponse,
    ValidationError as ApiValidationError, ValidationStats,
};

// Server
pub mod server {
    pub use crate::api::server::start_server;
}
