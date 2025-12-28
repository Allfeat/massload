//! JSON Schema validation for MIDDS musical works.
//!
//! This module provides validation functions for both flat (CSV row) and
//! grouped (complete work) MIDDS formats using JSON Schema Draft 7.
//!
//! # Validation Modes
//!
//! ## Flat Schema (CSV row)
//! - One creator per row
//! - Used for validating transformed CSV records
//! - Fields: `iswc`, `title`, `creatorIpi`, `creatorRole`
//!
//! ## Grouped Schema (complete work)
//! - Multiple creators in array
//! - Final MIDDS format for blockchain
//! - Fields: `iswc`, `title`, `creators[]`, `workType`
//!
//! # Embedded Schemas
//!
//! Schemas are embedded at compile time from `schemas/` directory:
//! - `midds-musical-work-flat.json`
//! - `midds-musical-work-grouped.json`

use serde_json::Value;

// Embedded schemas from workspace root
const SCHEMA_FLAT: &str = include_str!("../../../schemas/midds-musical-work-flat.json");
const SCHEMA_GROUPED: &str = include_str!("../../../schemas/midds-musical-work-grouped.json");

/// Validate a JSON object against a JSON schema.
///
/// # Arguments
/// * `schema` - The JSON schema (already parsed)
/// * `data` - The object to validate
///
/// # Returns
/// * `Ok(())` if valid
/// * `Err(Vec<String>)` with errors if invalid
pub fn validate(schema: &Value, data: &Value) -> Result<(), Vec<String>> {
    let validator =
        jsonschema::draft7::new(schema).map_err(|e| vec![format!("Invalid schema: {}", e)])?;

    let errors: Vec<String> = validator.iter_errors(data).map(|e| e.to_string()).collect();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Simple validation: returns true/false.
pub fn is_valid(schema: &Value, data: &Value) -> bool {
    jsonschema::draft7::is_valid(schema, data)
}

/// Validate against the grouped MIDDS schema (full work with creators array).
pub fn validate_musical_work_grouped(data: &Value) -> Result<(), Vec<String>> {
    let schema: Value = serde_json::from_str(SCHEMA_GROUPED).expect("Invalid embedded schema");
    validate(&schema, data)
}

/// Quick check against the grouped schema.
pub fn is_valid_musical_work_grouped(data: &Value) -> bool {
    let schema: Value = serde_json::from_str(SCHEMA_GROUPED).expect("Invalid embedded schema");
    is_valid(&schema, data)
}

/// Validate against the flat MIDDS schema (single row, one creator per row).
pub fn validate_musical_work_flat(data: &Value) -> Result<(), Vec<String>> {
    let schema: Value = serde_json::from_str(SCHEMA_FLAT).expect("Invalid embedded schema");
    validate(&schema, data)
}

/// Quick check against the flat schema.
pub fn is_valid_musical_work_flat(data: &Value) -> bool {
    let schema: Value = serde_json::from_str(SCHEMA_FLAT).expect("Invalid embedded schema");
    is_valid(&schema, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_grouped() {
        let work = json!({
            "iswc": "T1234567890",
            "title": "My Song",
            "creators": [{ "id": { "type": "Ipi", "value": 123456789 }, "role": "Composer" }],
            "participants": []
        });
        assert!(is_valid_musical_work_grouped(&work));
    }

    #[test]
    fn test_invalid_grouped() {
        let work = json!({
            "iswc": "INVALID",
            "title": "Test",
            "creators": []
        });
        assert!(!is_valid_musical_work_grouped(&work));
    }

    #[test]
    fn test_valid_flat() {
        let row = json!({
            "iswc": "T1234567890",
            "title": "My Song",
            "creatorIpi": 123456789,
            "creatorRole": "Composer"
        });
        assert!(is_valid_musical_work_flat(&row));
    }

    #[test]
    fn test_invalid_flat() {
        let row = json!({
            "iswc": "BAD",
            "title": "Test",
            "creatorIpi": 123,
            "creatorRole": "InvalidRole"
        });
        assert!(!is_valid_musical_work_flat(&row));
    }
}

