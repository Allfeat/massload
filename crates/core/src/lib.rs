//! # Allfeat Core
//!
//! Core domain models, validation, and error types for the Allfeat ecosystem.
//!
//! This crate provides:
//! - MIDDS domain models (Musical Works, Recordings, Releases, Parties)
//! - JSON Schema validation
//! - Error types used across the ecosystem
//!
//! ## Modules
//!
//! - [`models`] - Domain models (PartyId, Creator, MusicalWork, etc.)
//! - [`validation`] - JSON Schema validators for MIDDS
//! - [`error`] - Error types for the transformation pipeline

pub mod error;
pub mod models;

#[cfg(feature = "validation")]
pub mod validation;

// Re-exports - Models
pub use models::{
    Creator, CreatorRole, PartyId, MusicalWork, WorkType, ClassicalInfo,
};

// Legacy aliases for backward compatibility
#[cfg(not(target_arch = "wasm32"))]
pub use models::{CisacRole, GroupedWork, MusicalWorkType};

// Re-exports - Validation (only with "validation" feature)
#[cfg(feature = "validation")]
pub use validation::{
    is_valid, is_valid_musical_work_flat, is_valid_musical_work_grouped, validate,
    validate_musical_work_flat, validate_musical_work_grouped,
};

// Re-exports - Errors
pub use error::{
    AiError, CsvError, PipelineError, RegistryError, ServerError, TransformError, ValidationError,
};

