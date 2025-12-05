//! Transform flat musical work rows into grouped MIDDS format.
//!
//! This module handles the critical step of grouping multiple CSV rows
//! (one per creator) into complete musical works with all creators combined.
//!
//! # Architecture
//!
//! ```text
//! CSV Input (flat rows)          →  Grouped Output (SDK format)
//! ┌────────────────────────┐       ┌─────────────────────────┐
//! │ ISWC: T123, Creator: A │       │ ISWC: T123              │
//! │ ISWC: T123, Creator: B │  →    │ Creators: [A, B]        │
//! │ ISWC: T456, Creator: C │       ├─────────────────────────┤
//! └────────────────────────┘       │ ISWC: T456              │
//!                                  │ Creators: [C]           │
//!                                  └─────────────────────────┘
//! ```
//!
//! # Output Format
//!
//! The output follows the @allfeat/client SDK (dedot) format:
//!
//! - `creators[].id`: `{ "type": "Ipi", "value": 123 }` (dedot enum format)
//! - `creators[].role`: `"Composer"` (simple string)
//! - Optional fields are OMITTED if null (SDK doesn't like null)

use serde_json::{json, Map, Value};
use std::collections::HashMap;

/// Transform a set of flat rows into grouped musical works.
///
/// Output format is compatible with @allfeat/client SDK (dedot).
pub fn flat_to_grouped(flat_rows: Vec<Value>) -> Vec<Value> {
    let mut works: HashMap<String, WorkBuilder> = HashMap::new();

    for row in flat_rows {
        if let Some(iswc) = row.get("iswc").and_then(|v| v.as_str()) {
            let builder = works.entry(iswc.to_string()).or_insert_with(|| {
                WorkBuilder::new(&row)
            });
            builder.add_creator(&row);
        }
    }

    works.into_values().map(|b| b.build()).collect()
}

/// Builder for accumulating creators while grouping.
struct WorkBuilder {
    iswc: String,
    title: String,
    creation_year: Option<i64>,
    instrumental: Option<bool>,
    language: Option<String>,
    bpm: Option<i64>,
    key: Option<String>,
    work_type: Option<String>,
    opus: Option<String>,
    catalog_number: Option<String>,
    number_of_voices: Option<i64>,
    creators: Vec<Value>,
}

impl WorkBuilder {
    fn new(row: &Value) -> Self {
        Self {
            iswc: row.get("iswc").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            title: row.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            creation_year: row.get("creationYear").and_then(|v| v.as_i64()),
            instrumental: row.get("instrumental").and_then(|v| v.as_bool()),
            language: row.get("language").and_then(|v| v.as_str()).map(String::from),
            bpm: row.get("bpm").and_then(|v| v.as_i64()),
            key: row.get("key").and_then(|v| v.as_str()).map(String::from),
            work_type: row.get("workType").and_then(|v| v.as_str()).map(String::from),
            opus: row.get("opus").and_then(|v| v.as_str()).map(String::from),
            catalog_number: row.get("catalogNumber").and_then(|v| v.as_str()).map(String::from),
            number_of_voices: row.get("numberOfVoices").and_then(|v| v.as_i64()),
            creators: Vec::new(),
        }
    }

    fn add_creator(&mut self, row: &Value) {
        let ipi = row.get("creatorIpi").and_then(|v| v.as_i64());
        let isni = row.get("creatorIsni").and_then(|v| v.as_str());
        let role = row.get("creatorRole").and_then(|v| v.as_str());

        if let Some(role) = role {
            // Format SDK dedot: { "type": "Ipi", "value": 123 }
            let id = match (ipi, isni) {
                (Some(ipi), Some(isni)) => json!({
                    "type": "Both",
                    "value": { "ipi": ipi, "isni": isni }
                }),
                (Some(ipi), None) => json!({
                    "type": "Ipi",
                    "value": ipi
                }),
                (None, Some(isni)) => json!({
                    "type": "Isni",
                    "value": isni
                }),
                (None, None) => return, // Skip if no ID
            };

            // Role: simple string (SDK accepts this)
            self.creators.push(json!({
                "id": id,
                "role": role
            }));
        }
    }

    fn build(self) -> Value {
        let mut obj = Map::new();
        
        // Required fields
        obj.insert("iswc".to_string(), json!(self.iswc));
        obj.insert("title".to_string(), json!(self.title));
        obj.insert("creators".to_string(), json!(self.creators));
        
        // participants: required by Melodie runtime (empty array for now)
        // This field is for performers/interpreters, not creators
        obj.insert("participants".to_string(), json!([]));
        
        // Optional fields - ONLY include if present (SDK doesn't like null)
        if let Some(v) = self.creation_year {
            obj.insert("creationYear".to_string(), json!(v));
        }
        if let Some(v) = self.instrumental {
            obj.insert("instrumental".to_string(), json!(v));
        }
        if let Some(ref v) = self.language {
            obj.insert("language".to_string(), json!(v));
        }
        if let Some(v) = self.bpm {
            obj.insert("bpm".to_string(), json!(v));
        }
        if let Some(ref v) = self.key {
            obj.insert("key".to_string(), json!(v));
        }
        if let Some(ref v) = self.work_type {
            // workType: { type: "Original" } format for SDK
            obj.insert("workType".to_string(), json!({ "type": v }));
        }
        
        // Classical info - only if any field present
        if self.opus.is_some() || self.catalog_number.is_some() || self.number_of_voices.is_some() {
            let mut classical = Map::new();
            if let Some(v) = self.opus {
                classical.insert("opus".to_string(), json!(v));
            }
            if let Some(v) = self.catalog_number {
                classical.insert("catalogNumber".to_string(), json!(v));
            }
            if let Some(v) = self.number_of_voices {
                classical.insert("numberOfVoices".to_string(), json!(v));
            }
            obj.insert("classicalInfo".to_string(), Value::Object(classical));
        }

        Value::Object(obj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_single_work_multiple_creators() {
        let rows = vec![
            json!({
                "iswc": "T1234567890",
                "title": "My Song",
                "creationYear": 2024,
                "language": "English",
                "creatorIpi": 123456789,
                "creatorRole": "Composer"
            }),
            json!({
                "iswc": "T1234567890",
                "title": "My Song",
                "creationYear": 2024,
                "language": "English",
                "creatorIpi": 123456789,
                "creatorRole": "Author"
            }),
        ];

        let grouped = flat_to_grouped(rows);
        
        assert_eq!(grouped.len(), 1);
        let work = &grouped[0];
        assert_eq!(work["iswc"], "T1234567890");
        assert_eq!(work["title"], "My Song");
        assert_eq!(work["creators"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_sdk_format_ipi() {
        let rows = vec![
            json!({
                "iswc": "T1234567890",
                "title": "Test",
                "creatorIpi": 123,
                "creatorRole": "Composer"
            }),
        ];

        let grouped = flat_to_grouped(rows);
        let creator = &grouped[0]["creators"][0];
        
        // SDK format: { type: "Ipi", value: 123 }
        assert_eq!(creator["id"]["type"], "Ipi");
        assert_eq!(creator["id"]["value"], 123);
        assert_eq!(creator["role"], "Composer");
    }

    #[test]
    fn test_sdk_format_both() {
        let rows = vec![
            json!({
                "iswc": "T1234567890",
                "title": "Test",
                "creatorIpi": 123,
                "creatorIsni": "0000000121464388",
                "creatorRole": "Composer"
            }),
        ];

        let grouped = flat_to_grouped(rows);
        let creator = &grouped[0]["creators"][0];
        
        // SDK format: { type: "Both", value: { ipi: ..., isni: ... } }
        assert_eq!(creator["id"]["type"], "Both");
        assert_eq!(creator["id"]["value"]["ipi"], 123);
        assert_eq!(creator["id"]["value"]["isni"], "0000000121464388");
    }

    #[test]
    fn test_no_null_fields() {
        let rows = vec![
            json!({
                "iswc": "T1234567890",
                "title": "Test",
                "creatorIpi": 123,
                "creatorRole": "Composer"
            }),
        ];

        let grouped = flat_to_grouped(rows);
        let work = &grouped[0];
        
        // These optional fields should NOT be present (not even as null)
        assert!(work.get("language").is_none());
        assert!(work.get("bpm").is_none());
        assert!(work.get("key").is_none());
        assert!(work.get("classicalInfo").is_none());
        assert!(work.get("workType").is_none()); // Only if specified
    }

    #[test]
    fn test_work_type_format() {
        let rows = vec![
            json!({
                "iswc": "T1234567890",
                "title": "Test",
                "workType": "Original",
                "creatorIpi": 123,
                "creatorRole": "Composer"
            }),
        ];

        let grouped = flat_to_grouped(rows);
        
        // SDK format: { type: "Original" }
        assert_eq!(grouped[0]["workType"]["type"], "Original");
    }
}
