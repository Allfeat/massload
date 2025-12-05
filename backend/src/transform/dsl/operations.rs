//! DSL Operations for data transformation
//! 
//! Available operations that can be applied to transform CSV values into MIDDS-compliant data.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// All available transformation operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Operation {
    /// Remove leading and trailing whitespace
    Trim,
    
    /// Convert to uppercase
    Uppercase,
    
    /// Convert to lowercase
    Lowercase,
    
    /// Replace using regex pattern
    Replace {
        pattern: String,
        #[serde(default)]
        value: String,
    },
    
    /// Pad string at start to reach target length
    PadStart {
        length: usize,
        #[serde(default = "default_pad_char")]
        char: String,
    },
    
    /// Pad string at end to reach target length
    PadEnd {
        length: usize,
        #[serde(default = "default_pad_char")]
        char: String,
    },
    
    /// Extract year (4 digits) from a date string
    ExtractYear,
    
    /// Ensure string starts with given prefix
    EnsurePrefix {
        value: String,
    },
    
    /// Ensure string ends with given suffix
    EnsureSuffix {
        value: String,
    },
    
    /// Map values using a lookup table
    Map {
        mapping: HashMap<String, String>,
        #[serde(default)]
        case_insensitive: bool,
        /// Value to use when no mapping match found (null = return empty/drop field)
        #[serde(default)]
        default_unmapped: Option<String>,
    },
    
    /// Split string into array
    Split {
        #[serde(default = "default_split_separator")]
        separator: String,
    },
    
    /// Convert to boolean
    ToBoolean {
        #[serde(default = "default_true_values")]
        true_values: Vec<String>,
    },
    
    /// Convert to number (integer)
    ToNumber,
    
    /// Take first N characters
    Substring {
        start: usize,
        #[serde(default)]
        length: Option<usize>,
    },
    
    /// Remove all non-alphanumeric characters
    Alphanumeric,
    
    /// Remove all non-digit characters
    DigitsOnly,
}

fn default_pad_char() -> String {
    "0".to_string()
}

fn default_split_separator() -> String {
    ",".to_string()
}

fn default_true_values() -> Vec<String> {
    vec![
        "true".to_string(),
        "1".to_string(),
        "yes".to_string(),
        "oui".to_string(),
        "o".to_string(),
        "y".to_string(),
    ]
}

impl Operation {
    /// Apply this operation to a value
    pub fn apply(&self, value: &Value) -> Value {
        match self {
            Operation::Trim => self.apply_trim(value),
            Operation::Uppercase => self.apply_uppercase(value),
            Operation::Lowercase => self.apply_lowercase(value),
            Operation::Replace { pattern, value: replacement } => {
                self.apply_replace(value, pattern, replacement)
            }
            Operation::PadStart { length, char } => self.apply_pad_start(value, *length, char),
            Operation::PadEnd { length, char } => self.apply_pad_end(value, *length, char),
            Operation::ExtractYear => self.apply_extract_year(value),
            Operation::EnsurePrefix { value: prefix } => self.apply_ensure_prefix(value, prefix),
            Operation::EnsureSuffix { value: suffix } => self.apply_ensure_suffix(value, suffix),
            Operation::Map { mapping, case_insensitive, default_unmapped } => {
                self.apply_map(value, mapping, *case_insensitive, default_unmapped.as_deref())
            }
            Operation::Split { separator } => self.apply_split(value, separator),
            Operation::ToBoolean { true_values } => self.apply_to_boolean(value, true_values),
            Operation::ToNumber => self.apply_to_number(value),
            Operation::Substring { start, length } => self.apply_substring(value, *start, *length),
            Operation::Alphanumeric => self.apply_alphanumeric(value),
            Operation::DigitsOnly => self.apply_digits_only(value),
        }
    }

    fn as_string(value: &Value) -> Option<String> {
        match value {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            _ => None,
        }
    }

    fn apply_trim(&self, value: &Value) -> Value {
        Self::as_string(value)
            .map(|s| Value::String(s.trim().to_string()))
            .unwrap_or(value.clone())
    }

    fn apply_uppercase(&self, value: &Value) -> Value {
        Self::as_string(value)
            .map(|s| Value::String(s.to_uppercase()))
            .unwrap_or(value.clone())
    }

    fn apply_lowercase(&self, value: &Value) -> Value {
        Self::as_string(value)
            .map(|s| Value::String(s.to_lowercase()))
            .unwrap_or(value.clone())
    }

    fn apply_replace(&self, value: &Value, pattern: &str, replacement: &str) -> Value {
        Self::as_string(value)
            .and_then(|s| {
                regex::Regex::new(pattern)
                    .ok()
                    .map(|re| Value::String(re.replace_all(&s, replacement).to_string()))
            })
            .unwrap_or(value.clone())
    }

    fn apply_pad_start(&self, value: &Value, length: usize, pad_char: &str) -> Value {
        Self::as_string(value)
            .map(|s| {
                if s.len() >= length {
                    Value::String(s)
                } else {
                    let pad = pad_char.chars().next().unwrap_or('0');
                    let padding: String = std::iter::repeat_n(pad, length - s.len()).collect();
                    Value::String(format!("{}{}", padding, s))
                }
            })
            .unwrap_or(value.clone())
    }

    fn apply_pad_end(&self, value: &Value, length: usize, pad_char: &str) -> Value {
        Self::as_string(value)
            .map(|s| {
                if s.len() >= length {
                    Value::String(s)
                } else {
                    let pad = pad_char.chars().next().unwrap_or('0');
                    let padding: String = std::iter::repeat_n(pad, length - s.len()).collect();
                    Value::String(format!("{}{}", s, padding))
                }
            })
            .unwrap_or(value.clone())
    }

    fn apply_extract_year(&self, value: &Value) -> Value {
        Self::as_string(value)
            .and_then(|s| {
                // Try to find 4 consecutive digits
                regex::Regex::new(r"\d{4}")
                    .ok()
                    .and_then(|re| re.find(&s).map(|m| m.as_str().to_string()))
                    .and_then(|year| year.parse::<i64>().ok())
                    .map(|n| Value::Number(n.into()))
            })
            .unwrap_or(Value::Null)
    }

    fn apply_ensure_prefix(&self, value: &Value, prefix: &str) -> Value {
        Self::as_string(value)
            .map(|s| {
                if s.starts_with(prefix) {
                    Value::String(s)
                } else {
                    Value::String(format!("{}{}", prefix, s))
                }
            })
            .unwrap_or(value.clone())
    }

    fn apply_ensure_suffix(&self, value: &Value, suffix: &str) -> Value {
        Self::as_string(value)
            .map(|s| {
                if s.ends_with(suffix) {
                    Value::String(s)
                } else {
                    Value::String(format!("{}{}", s, suffix))
                }
            })
            .unwrap_or(value.clone())
    }

    fn apply_map(&self, value: &Value, mapping: &HashMap<String, String>, case_insensitive: bool, default_unmapped: Option<&str>) -> Value {
        Self::as_string(value)
            .map(|s| {
                let key = if case_insensitive { s.to_lowercase() } else { s.clone() };
                
                let found = if case_insensitive {
                    mapping.iter().find(|(k, _)| k.to_lowercase() == key)
                } else {
                    mapping.get_key_value(&key)
                };
                
                match found {
                    Some((_, v)) => Value::String(v.clone()),
                    None => {
                        // No match found - use default or return empty (which will drop the field)
                        match default_unmapped {
                            Some(d) => Value::String(d.to_string()),
                            None => Value::String(String::new()), // Empty = field will be dropped
                        }
                    }
                }
            })
            .unwrap_or(value.clone())
    }

    fn apply_split(&self, value: &Value, separator: &str) -> Value {
        Self::as_string(value)
            .map(|s| {
                let parts: Vec<Value> = s
                    .split(separator)
                    .map(|p| Value::String(p.trim().to_string()))
                    .collect();
                Value::Array(parts)
            })
            .unwrap_or(value.clone())
    }

    fn apply_to_boolean(&self, value: &Value, true_values: &[String]) -> Value {
        match value {
            Value::Bool(b) => Value::Bool(*b),
            _ => Self::as_string(value)
                .map(|s| {
                    let lower = s.to_lowercase();
                    Value::Bool(true_values.iter().any(|tv| tv.to_lowercase() == lower))
                })
                .unwrap_or(Value::Bool(false)),
        }
    }

    fn apply_to_number(&self, value: &Value) -> Value {
        match value {
            Value::Number(_) => value.clone(),
            _ => Self::as_string(value)
                .and_then(|s| {
                    // Check if starts with minus for negative numbers
                    let is_negative = s.trim().starts_with('-');
                    // Keep only digits
                    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                    if digits.is_empty() {
                        return None;
                    }
                    let num_str = if is_negative {
                        format!("-{}", digits)
                    } else {
                        digits
                    };
                    num_str.parse::<i64>().ok().map(|n| Value::Number(n.into()))
                })
                .unwrap_or(Value::Null),
        }
    }

    fn apply_substring(&self, value: &Value, start: usize, length: Option<usize>) -> Value {
        Self::as_string(value)
            .map(|s| {
                let chars: Vec<char> = s.chars().collect();
                let end = length.map(|l| start + l).unwrap_or(chars.len());
                let result: String = chars.get(start..end.min(chars.len()))
                    .map(|c| c.iter().collect())
                    .unwrap_or_default();
                Value::String(result)
            })
            .unwrap_or(value.clone())
    }

    fn apply_alphanumeric(&self, value: &Value) -> Value {
        Self::as_string(value)
            .map(|s| {
                let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
                Value::String(cleaned)
            })
            .unwrap_or(value.clone())
    }

    fn apply_digits_only(&self, value: &Value) -> Value {
        Self::as_string(value)
            .map(|s| {
                let cleaned: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                Value::String(cleaned)
            })
            .unwrap_or(value.clone())
    }
}

/// Get a description of all available operations for AI prompts
pub fn operations_description() -> String {
    r#"Available transformation operations:

| Operation | Description | Parameters |
|-----------|-------------|------------|
| trim | Remove leading/trailing whitespace | - |
| uppercase | Convert to uppercase | - |
| lowercase | Convert to lowercase | - |
| replace | Regex pattern replacement | pattern: regex, value: replacement |
| pad_start | Pad string at start | length: target length, char: pad character (default "0") |
| pad_end | Pad string at end | length: target length, char: pad character (default "0") |
| extract_year | Extract 4-digit year from date | - |
| ensure_prefix | Add prefix if not present | value: prefix string |
| ensure_suffix | Add suffix if not present | value: suffix string |
| map | Map values using lookup table | mapping: {source: target}, case_insensitive: bool |
| split | Split into array | separator: split char (default ",") |
| to_boolean | Convert to boolean | true_values: list of truthy strings |
| to_number | Convert to integer | - |
| substring | Extract substring | start: start index, length: optional length |
| alphanumeric | Keep only alphanumeric chars | - |
| digits_only | Keep only digits | - |

Example operations in JSON:
[
  {"type": "trim"},
  {"type": "replace", "pattern": "[-. ]", "value": ""},
  {"type": "map", "mapping": {"CA": "Composer", "A": "Author"}, "case_insensitive": true},
  {"type": "to_number"},
  {"type": "ensure_prefix", "value": "T"}
]"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim() {
        let op = Operation::Trim;
        assert_eq!(op.apply(&Value::String("  hello  ".to_string())), Value::String("hello".to_string()));
    }

    #[test]
    fn test_map() {
        let mut mapping = HashMap::new();
        mapping.insert("CA".to_string(), "Composer".to_string());
        mapping.insert("A".to_string(), "Author".to_string());
        
        let op = Operation::Map { mapping: mapping.clone(), case_insensitive: true, default_unmapped: None };
        assert_eq!(op.apply(&Value::String("ca".to_string())), Value::String("Composer".to_string()));
        
        // Test no match returns empty string (will drop field)
        assert_eq!(op.apply(&Value::String("Unknown".to_string())), Value::String(String::new()));
        
        // Test with default
        let op_with_default = Operation::Map { mapping, case_insensitive: true, default_unmapped: Some("Other".to_string()) };
        assert_eq!(op_with_default.apply(&Value::String("Unknown".to_string())), Value::String("Other".to_string()));
    }

    #[test]
    fn test_to_number() {
        let op = Operation::ToNumber;
        assert_eq!(op.apply(&Value::String("123456789".to_string())), Value::Number(123456789.into()));
        assert_eq!(op.apply(&Value::String("123-456-789".to_string())), Value::Number(123456789.into()));
    }

    #[test]
    fn test_extract_year() {
        let op = Operation::ExtractYear;
        assert_eq!(op.apply(&Value::String("15/03/2024".to_string())), Value::Number(2024.into()));
        assert_eq!(op.apply(&Value::String("2023-12-25".to_string())), Value::Number(2023.into()));
    }

    #[test]
    fn test_ensure_prefix() {
        let op = Operation::EnsurePrefix { value: "T".to_string() };
        assert_eq!(op.apply(&Value::String("1234567890".to_string())), Value::String("T1234567890".to_string()));
        assert_eq!(op.apply(&Value::String("T1234567890".to_string())), Value::String("T1234567890".to_string()));
    }
}

