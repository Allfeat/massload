//! Transformation module.
//!
//! This module handles CSV to MIDDS transformation:
//! - DSL: Transformation operations and matrix
//! - Grouper: Flat rows to grouped works
//! - Pipeline: Main transformation pipeline

pub mod dsl;
pub mod grouper;
pub mod pipeline;

pub use dsl::*;
pub use grouper::flat_to_grouped;
pub use pipeline::*;

