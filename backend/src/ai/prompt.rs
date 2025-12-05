//! Prompt generation for AI matrix generation
//!
//! Builds prompts to send CSV data to the AI and get back transformation matrices.

use serde_json::Value;

/// The transformation matrix JSON schema (embedded at compile time)
const MATRIX_SCHEMA: &str = include_str!("../../schemas/transformation-matrix-schema.json");

/// Generate the system prompt for matrix generation
pub fn system_prompt() -> String {
    format!(
        r#"You are a data transformation expert. Your task is to analyze CSV data and generate a transformation matrix that converts raw CSV columns into standardized MIDDS (Music Industry Data Description Standard) format.

## Your Mission

Given:
1. A preview of CSV data (as JSON objects)
2. The MIDDS flat schema (target format)
3. The transformation matrix JSON schema (your output format)

You must return a valid JSON transformation matrix that maps CSV columns to MIDDS fields.

## CRITICAL: Output Format

You MUST return ONLY valid JSON matching this schema EXACTLY:

```json
{matrix_schema}
```

## MIDDS Field Requirements

### Required fields (must be mapped):
- `iswc`: International Standard Musical Work Code. Format: T + 10 digits (e.g., "T1234567890")
- `title`: Title of the work (string, max 256 chars)
- `creatorIpi`: IPI code (integer, 9-11 digits)
- `creatorRole`: Must be one of: "Author", "Composer", "Arranger", "Adapter", "Publisher"

### Optional fields:
- `creationYear`: 4-digit year (integer)
- `instrumental`: boolean
- `language`: Must be one of: "English", "French", "Spanish", "German", "Italian", "Portuguese", "Russian", "Chinese", "Japanese", "Korean", "Arabic", "Hindi", "Dutch", "Swedish", "Norwegian", "Finnish", "Polish", "Turkish", "Hebrew", "Greek", "Latin", "Esperanto"
- `bpm`: beats per minute (integer)
- `key`: Musical key (e.g., "Am", "C", "Fs", "Bb", "Dm", etc.)
- `workType`: Type of work - MUST be "Original" or null. Map any column containing work type info.
- `creatorIsni`: 16-character ISNI code (format: 16 digits/X)
- `opus`, `catalogNumber`, `numberOfVoices`: For classical works

## Work Type Mapping

If the CSV has a column for work type (e.g., "Work Type", "Type", "Type d'oeuvre"), map it to `workType`:
- "Original", "Orig", "O", "original" → "Original"
- "Medley", "Mashup", "Adaptation", or any other value → null (not supported in flat format)
- Empty or missing → null

## Role Code Mapping

Common role codes to map:
- CA, C+A → Both Composer and Author (map to "Composer" for now)
- C, Comp, Komponist → "Composer"  
- A, Autor, Textdichter, Lyricist → "Author"
- AR, Arr, Arrangeur → "Arranger"
- AD, Adapt → "Adapter"
- E, Ed, Pub, Publisher, Verlag, Editeur → "Publisher"

## Rules

1. Use ONLY operations defined in the schema: trim, uppercase, lowercase, replace, pad_start, pad_end, extract_year, ensure_prefix, ensure_suffix, map, split, to_boolean, to_number, substring, alphanumeric, digits_only
2. Do NOT invent new operations
3. Use exact CSV column names from the preview (case-sensitive)
4. Always use `trim` for text fields
5. Always use `to_number` for IPI codes
6. Use `map` operation for role codes, language translations, and workType
7. For ISWC: remove punctuation with `replace`, ensure "T" prefix with `ensure_prefix`
8. For workType: use `map` to convert CSV values to "Original" (only valid value) or omit invalid types
9. MAP ALL COLUMNS that correspond to MIDDS fields - do not skip any mappable columns!
10. Return ONLY the JSON object, no explanations or markdown"#,
        matrix_schema = MATRIX_SCHEMA
    )
}

/// Generate the user prompt with CSV data and schema
/// 
/// # Arguments
/// * `csv_preview` - First N rows for the AI to see the structure
/// * `all_records` - All records (for extracting unique values)
/// * `schema` - Target MIDDS schema
pub fn user_prompt_with_all_data(csv_preview: &[Value], all_records: &[Value], schema: &Value) -> String {
    let preview_json = serde_json::to_string_pretty(csv_preview).unwrap_or_default();
    let schema_json = serde_json::to_string_pretty(schema).unwrap_or_default();

    // Extract unique values from ALL records, not just preview
    let unique_values = extract_unique_values(all_records);

    let preview_count = csv_preview.len();
    let total_count = all_records.len();
    
    format!(
        r#"## CSV Data Preview ({preview_count} rows shown, {total_count} total)

```json
{preview_json}
```

## ALL Unique Values per Column (from {total_count} rows - IMPORTANT for mapping)

{unique_values}

## Target MIDDS Flat Schema

```json
{schema_json}
```

## Task

Analyze the CSV columns and generate a transformation matrix.
Map ALL unique values you see above (especially for Role column - map ALL role codes!).

Return ONLY the JSON object matching the transformation matrix schema. No explanations."#
    )
}


/// Extract unique values per column for AI analysis
fn extract_unique_values(rows: &[Value]) -> String {
    use std::collections::{HashMap, HashSet};

    let mut column_values: HashMap<String, HashSet<String>> = HashMap::new();

    for row in rows {
        if let Some(obj) = row.as_object() {
            for (key, value) in obj {
                let entry = column_values.entry(key.clone()).or_default();
                if let Some(s) = value.as_str() {
                    // No limit - collect ALL unique values
                    entry.insert(s.to_string());
                }
            }
        }
    }

    let mut result = String::new();
    let mut columns: Vec<_> = column_values.iter().collect();
    columns.sort_by_key(|(k, _)| k.as_str());
    
    // Columns likely to need full mapping (show ALL values)
    let mapping_columns = ["role", "genre", "type", "instrumental", "language", "société"];
    
    for (col, values) in columns {
        let mut values_vec: Vec<&str> = values.iter().map(|s| s.as_str()).collect();
        values_vec.sort();
        
        let col_lower = col.to_lowercase();
        let is_mapping_column = mapping_columns.iter().any(|&m| col_lower.contains(m));
        
        let display = if is_mapping_column || values_vec.len() <= 30 {
            // Show ALL unique values for mapping columns or low cardinality
            if values_vec.len() > 50 {
                format!(
                    "{} ({} unique values)",
                    values_vec.join(", "),
                    values_vec.len()
                )
            } else {
                values_vec.join(", ")
            }
        } else {
            // High cardinality column (like names, titles) - show sample
            format!(
                "{}, ... ({} unique - high cardinality, sample shown)",
                values_vec[..15.min(values_vec.len())].join(", "),
                values_vec.len()
            )
        };
        
        result.push_str(&format!("- **{}**: {}\n", col, display));
    }

    result
}

/// Build the complete prompt for streaming (with all data for unique values)
pub fn build_messages_with_all_data(csv_preview: &[Value], all_records: &[Value], schema: &Value) -> Vec<serde_json::Value> {
    vec![serde_json::json!({
        "role": "user",
        "content": user_prompt_with_all_data(csv_preview, all_records, schema)
    })]
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_system_prompt_contains_schema() {
        let prompt = system_prompt();
        assert!(prompt.contains("\"type\": \"trim\""));
        assert!(prompt.contains("\"type\": \"map\""));
        assert!(prompt.contains("\"type\": \"to_number\""));
        assert!(prompt.contains("FieldTransform"));
    }

    #[test]
    fn test_user_prompt_includes_data() {
        let csv = vec![json!({"ISWC": "T1234567890", "TITRE": "Test"})];
        let schema = json!({"type": "object"});

        let prompt = user_prompt_with_all_data(&csv, &csv, &schema);
        assert!(prompt.contains("T1234567890"));
        assert!(prompt.contains("TITRE"));
    }

    #[test]
    fn test_matrix_schema_is_valid_json() {
        let schema: Value = serde_json::from_str(MATRIX_SCHEMA).expect("Schema should be valid JSON");
        assert!(schema.get("definitions").is_some());
        assert!(schema.get("properties").is_some());
    }
}
