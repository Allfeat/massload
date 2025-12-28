//! DSL (Domain Specific Language) for CSV to MIDDS transformation
//! 
//! This module provides:
//! - `matrix`: Transformation matrix definition (what AI returns)
//! - `operations`: Available transformation operations
//! - `executor`: Execute matrices on CSV data
//! 
//! ## Usage Flow
//! 
//! ```text
//! CSV → parse::csv_to_json → AI generates Matrix → executor::execute → Flat JSON → validate
//! ```
//! 
//! ## Example
//! 
//! ```rust,ignore
//! use massload::dsl::{TransformationMatrix, execute};
//! use massload::parse::csv_to_json;
//! use massload::validator::validate_musical_work_flat;
//! 
//! // 1. Parse CSV
//! let csv_data = csv_to_json(csv_content, ';').unwrap();
//! 
//! // 2. Load matrix (from AI or file)
//! let matrix = TransformationMatrix::from_json(matrix_json).unwrap();
//! 
//! // 3. Execute transformation
//! let result = execute(&csv_data, &matrix);
//! 
//! // 4. Validate each record
//! for record in result.records {
//!     match validate_musical_work_flat(&record) {
//!         Ok(()) => println!("Valid!"),
//!         Err(errors) => println!("Invalid: {:?}", errors),
//!     }
//! }
//! ```

pub mod executor;
pub mod matrix;
pub mod operations;

// Re-exports for convenience
pub use executor::{execute, execute_hashmap, SkippedRow, TransformError, TransformResult};
pub use matrix::{example_matrix, FieldTransform, SourceFormat, TransformationMatrix, ExpandConfig, ColumnVariant};
pub use operations::{operations_description, Operation};

