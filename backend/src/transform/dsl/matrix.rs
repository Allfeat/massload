//! Transformation Matrix definition
//! 
//! The matrix defines how to transform CSV columns into MIDDS flat fields.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::operations::Operation;

/// A complete transformation matrix defining all field transformations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationMatrix {
    /// Version of the matrix format
    #[serde(default = "default_version")]
    pub version: String,
    
    /// Human-readable description
    #[serde(default)]
    pub description: String,
    
    /// Source format metadata
    #[serde(default)]
    pub source_format: Option<SourceFormat>,
    
    /// Field transformations: key = target MIDDS field, value = transformation rule
    pub transforms: HashMap<String, FieldTransform>,
    
    /// Row expansion rules (one CSV row → multiple flat records)
    #[serde(default)]
    pub expand: Option<ExpandConfig>,
}

/// Configuration for expanding one CSV row into multiple flat records
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExpandConfig {
    /// Expand based on a role field containing combined values (e.g., "C+A")
    SplitRole {
        /// Column containing the combined role
        source: String,
        /// Separator between roles (default: "+")
        #[serde(default = "default_role_separator")]
        separator: String,
        /// Mapping from split values to MIDDS roles
        mapping: HashMap<String, String>,
    },
    
    /// Expand based on multiple column sets (e.g., Composer_IPI + Author_IPI)
    MultipleColumns {
        /// Each variant creates a separate record
        variants: Vec<ColumnVariant>,
    },
}

fn default_role_separator() -> String {
    "+".to_string()
}

/// A column variant for multi-column expansion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnVariant {
    /// Condition: only create record if this column is not empty
    pub condition_column: Option<String>,
    /// Override transforms for this variant
    #[serde(default)]
    pub overrides: HashMap<String, FieldTransform>,
}

fn default_version() -> String {
    "1.0".to_string()
}

/// Metadata about the source CSV format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFormat {
    /// Detected or specified delimiter
    pub delimiter: Option<char>,
    
    /// Detected or specified encoding
    pub encoding: Option<String>,
    
    /// Number of header rows
    #[serde(default = "default_header_rows")]
    pub header_rows: usize,
}

fn default_header_rows() -> usize {
    1
}

/// Transformation rule for a single field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldTransform {
    /// Source column name from CSV (mutually exclusive with sources and constant)
    #[serde(default)]
    pub source: Option<String>,
    
    /// Multiple source columns to concatenate (mutually exclusive with source and constant)
    #[serde(default)]
    pub sources: Option<Vec<String>>,
    
    /// Separator for concatenating multiple sources (default: " ")
    #[serde(default = "default_concat_separator")]
    pub concat_separator: String,
    
    /// Constant value (mutually exclusive with source/sources)
    #[serde(default)]
    pub constant: Option<Value>,
    
    /// Ordered list of operations to apply
    #[serde(default)]
    pub operations: Vec<Operation>,
    
    /// Default value if source is empty or transformation fails
    #[serde(default)]
    pub default: Option<Value>,
    
    /// Whether this field is required
    #[serde(default)]
    pub required: bool,
}

fn default_concat_separator() -> String {
    " ".to_string()
}

impl TransformationMatrix {
    /// Create an empty matrix
    pub fn new() -> Self {
        Self {
            version: default_version(),
            description: String::new(),
            source_format: None,
            transforms: HashMap::new(),
            expand: None,
        }
    }

    /// Parse a matrix from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Parse a matrix from JSON value
    pub fn from_value(value: &Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value.clone())
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Get all source columns referenced in the matrix
    pub fn source_columns(&self) -> Vec<String> {
        let mut columns: Vec<String> = self.transforms
            .values()
            .flat_map(|t| t.get_sources())
            .collect();
        
        // Add columns from expand config
        if let Some(ref expand) = self.expand {
            match expand {
                ExpandConfig::SplitRole { source, .. } => {
                    columns.push(source.clone());
                }
                ExpandConfig::MultipleColumns { variants } => {
                    for variant in variants {
                        if let Some(ref col) = variant.condition_column {
                            columns.push(col.clone());
                        }
                        for t in variant.overrides.values() {
                            columns.extend(t.get_sources());
                        }
                    }
                }
            }
        }
        
        // Deduplicate
        columns.sort();
        columns.dedup();
        columns
    }

    /// Get all target fields (MIDDS fields)
    pub fn target_fields(&self) -> Vec<String> {
        self.transforms.keys().cloned().collect()
    }

    /// Validate that all required source columns exist in the CSV headers
    pub fn validate_headers(&self, headers: &[String]) -> Result<(), Vec<String>> {
        let missing: Vec<String> = self
            .source_columns()
            .into_iter()
            .filter(|col| !headers.iter().any(|h| h == col))
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
}

impl Default for TransformationMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldTransform {
    /// Create a transform from a source column
    pub fn from_source(source: &str) -> Self {
        Self {
            source: Some(source.to_string()),
            sources: None,
            concat_separator: default_concat_separator(),
            constant: None,
            operations: Vec::new(),
            default: None,
            required: false,
        }
    }

    /// Create a transform from multiple source columns (concatenated)
    pub fn from_sources(sources: Vec<String>, separator: &str) -> Self {
        Self {
            source: None,
            sources: Some(sources),
            concat_separator: separator.to_string(),
            constant: None,
            operations: Vec::new(),
            default: None,
            required: false,
        }
    }

    /// Create a transform with a constant value
    pub fn from_constant(value: Value) -> Self {
        Self {
            source: None,
            sources: None,
            concat_separator: default_concat_separator(),
            constant: Some(value),
            operations: Vec::new(),
            default: None,
            required: false,
        }
    }

    /// Add an operation to the chain
    pub fn with_operation(mut self, op: Operation) -> Self {
        self.operations.push(op);
        self
    }

    /// Set the default value
    pub fn with_default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    /// Get all source columns referenced by this transform
    pub fn get_sources(&self) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(ref s) = self.source {
            result.push(s.clone());
        }
        if let Some(ref ss) = self.sources {
            result.extend(ss.clone());
        }
        result
    }
}

/// Generate an example matrix for documentation/AI prompts
pub fn example_matrix() -> TransformationMatrix {
    let mut transforms = HashMap::new();

    // ISWC transformation
    transforms.insert(
        "iswc".to_string(),
        FieldTransform::from_source("Code ISWC")
            .with_operation(Operation::Trim)
            .with_operation(Operation::Replace {
                pattern: "[-. ]".to_string(),
                value: "".to_string(),
            })
            .with_operation(Operation::EnsurePrefix {
                value: "T".to_string(),
            })
            .required(),
    );

    // Title transformation
    transforms.insert(
        "title".to_string(),
        FieldTransform::from_source("Titre")
            .with_operation(Operation::Trim)
            .required(),
    );

    // Creator Role transformation
    let mut role_mapping = HashMap::new();
    role_mapping.insert("CA".to_string(), "Composer".to_string());
    role_mapping.insert("A".to_string(), "Author".to_string());
    role_mapping.insert("AR".to_string(), "Arranger".to_string());

    transforms.insert(
        "creatorRole".to_string(),
        FieldTransform::from_source("Role")
            .with_operation(Operation::Trim)
            .with_operation(Operation::Uppercase)
            .with_operation(Operation::Map {
                mapping: role_mapping,
                case_insensitive: true,
                default_unmapped: None,
            })
            .with_default(Value::String("Composer".to_string()))
            .required(),
    );

    // Creator IPI
    transforms.insert(
        "creatorIpi".to_string(),
        FieldTransform::from_source("IPI")
            .with_operation(Operation::DigitsOnly)
            .with_operation(Operation::ToNumber)
            .required(),
    );

    // Instrumental flag
    transforms.insert(
        "instrumental".to_string(),
        FieldTransform::from_source("Instrumental")
            .with_operation(Operation::ToBoolean {
                true_values: vec![
                    "oui".to_string(),
                    "yes".to_string(),
                    "1".to_string(),
                    "true".to_string(),
                    "x".to_string(),
                ],
            })
            .with_default(Value::Bool(false)),
    );

    TransformationMatrix {
        version: "1.0".to_string(),
        description: "Example transformation matrix for MIDDS Musical Work".to_string(),
        source_format: Some(SourceFormat {
            delimiter: Some(';'),
            encoding: Some("utf-8".to_string()),
            header_rows: 1,
        }),
        transforms,
        expand: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_serialization() {
        let matrix = example_matrix();
        let json = matrix.to_json().unwrap();
        let parsed = TransformationMatrix::from_json(&json).unwrap();
        assert_eq!(parsed.version, matrix.version);
    }

    #[test]
    fn test_validate_headers() {
        let matrix = example_matrix();
        let headers = vec![
            "Code ISWC".to_string(),
            "Titre".to_string(),
            "Role".to_string(),
            "IPI".to_string(),
            "Instrumental".to_string(),
        ];
        assert!(matrix.validate_headers(&headers).is_ok());

        let missing_headers = vec!["Code ISWC".to_string()];
        let result = matrix.validate_headers(&missing_headers);
        assert!(result.is_err());
    }
}

