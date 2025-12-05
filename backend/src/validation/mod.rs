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
//!
//! # Example
//!
//! ```rust,ignore
//! use serde_json::json;
//! use massload::{validate_musical_work_flat, is_valid_musical_work_grouped};
//!
//! // Validate a flat record
//! let flat = json!({
//!     "iswc": "T1234567890",
//!     "title": "My Song",
//!     "creatorIpi": 123456789,
//!     "creatorRole": "Composer"
//! });
//! assert!(validate_musical_work_flat(&flat).is_ok());
//!
//! // Validate a grouped work (SDK format)
//! let grouped = json!({
//!     "iswc": "T1234567890",
//!     "title": "My Song",
//!     "creators": [{ "id": { "type": "Ipi", "value": 123456789 }, "role": "Composer" }],
//!     "participants": []
//! });
//! assert!(is_valid_musical_work_grouped(&grouped));
//! ```

use serde_json::Value;

/// Valide un objet JSON contre un schéma JSON.
///
/// # Arguments
/// * `schema` - Le schéma JSON (déjà parsé)
/// * `data` - L'objet à valider
///
/// # Returns
/// * `Ok(())` si valide
/// * `Err(Vec<String>)` avec les erreurs si invalide
///
/// # Example
/// ```ignore
/// use serde_json::json;
/// use massload::validation::validate;
///
/// let schema = json!({
///     "type": "object",
///     "required": ["name"],
///     "properties": {
///         "name": { "type": "string" }
///     }
/// });
///
/// let valid_data = json!({ "name": "test" });
/// assert!(validate(&schema, &valid_data).is_ok());
///
/// let invalid_data = json!({ "age": 42 });
/// assert!(validate(&schema, &invalid_data).is_err());
/// ```
pub fn validate(schema: &Value, data: &Value) -> Result<(), Vec<String>> {
    let validator = jsonschema::draft7::new(schema)
        .map_err(|e| vec![format!("Schéma invalide: {}", e)])?;

    let errors: Vec<String> = validator
        .iter_errors(data)
        .map(|e| e.to_string())
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Version encore plus simple : retourne juste true/false.
pub fn is_valid(schema: &Value, data: &Value) -> bool {
    jsonschema::draft7::is_valid(schema, data)
}

/// Validate against the grouped MIDDS schema (full work with creators array).
pub fn validate_musical_work_grouped(data: &Value) -> Result<(), Vec<String>> {
    let schema: Value = serde_json::from_str(include_str!("../../schemas/midds-musical-work-grouped.json"))
        .expect("Invalid embedded schema");
    validate(&schema, data)
}

/// Quick check against the grouped schema.
pub fn is_valid_musical_work_grouped(data: &Value) -> bool {
    let schema: Value = serde_json::from_str(include_str!("../../schemas/midds-musical-work-grouped.json"))
        .expect("Invalid embedded schema");
    is_valid(&schema, data)
}

/// Validate against the flat MIDDS schema (single row, one creator per row).
pub fn validate_musical_work_flat(data: &Value) -> Result<(), Vec<String>> {
    let schema: Value = serde_json::from_str(include_str!("../../schemas/midds-musical-work-flat.json"))
        .expect("Invalid embedded schema");
    validate(&schema, data)
}

/// Quick check against the flat schema.
pub fn is_valid_musical_work_flat(data: &Value) -> bool {
    let schema: Value = serde_json::from_str(include_str!("../../schemas/midds-musical-work-flat.json"))
        .expect("Invalid embedded schema");
    is_valid(&schema, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_grouped() {
        // SDK format: { "type": "Ipi", "value": ... }
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

    #[test]
    fn test_flat_with_errors() {
        let row = json!({ "iswc": "T1234567890" });
        let result = validate_musical_work_flat(&row);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(!errors.is_empty());
        println!("Errors: {:?}", errors);
    }
}
