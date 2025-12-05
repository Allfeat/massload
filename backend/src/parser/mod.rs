//! Generic CSV to JSON parser with encoding and delimiter auto-detection.
//!
//! Converts CSV rows into JSON objects. No MIDDS-specific logic here.

use serde_json::{json, Map, Value};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

/// CSV parsing error with context
#[derive(Debug, Clone)]
pub struct CsvError {
    pub line: usize,
    pub column: Option<String>,
    pub value: Option<String>,
    pub message: String,
}

impl std::fmt::Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.column, &self.value) {
            (Some(col), Some(val)) => {
                write!(f, "Line {}, column '{}' (value '{}'): {}", self.line, col, val, self.message)
            }
            (Some(col), None) => {
                write!(f, "Line {}, column '{}': {}", self.line, col, self.message)
            }
            _ => {
                write!(f, "Line {}: {}", self.line, self.message)
            }
        }
    }
}

impl std::error::Error for CsvError {}

impl CsvError {
    pub fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            column: None,
            value: None,
            message: message.into(),
        }
    }

    pub fn with_column(mut self, column: impl Into<String>) -> Self {
        self.column = Some(column.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

/// Result of parsing with metadata
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// Parsed records as JSON objects
    pub records: Vec<Value>,
    /// Detected or used encoding
    pub encoding: String,
    /// Detected or used delimiter
    pub delimiter: char,
    /// Column headers
    pub headers: Vec<String>,
}

/// Detect the encoding of raw bytes using chardet
pub fn detect_encoding(bytes: &[u8]) -> String {
    let result = chardet::detect(bytes);
    let charset = result.0;
    
    // Normalize charset names
    match charset.to_lowercase().as_str() {
        "ascii" | "utf-8" | "utf8" => "utf-8".to_string(),
        "iso-8859-1" | "iso-8859-15" | "latin-1" | "latin1" => "iso-8859-1".to_string(),
        "windows-1252" | "cp1252" => "windows-1252".to_string(),
        _ => charset,
    }
}

/// Decode bytes to string using the specified encoding
pub fn decode_content(bytes: &[u8], encoding: &str) -> Result<String, CsvError> {
    match encoding.to_lowercase().as_str() {
        "utf-8" | "utf8" | "ascii" => {
            String::from_utf8(bytes.to_vec())
                .or_else(|_| Ok(String::from_utf8_lossy(bytes).to_string()))
        }
        "iso-8859-1" | "latin-1" | "latin1" => {
            Ok(encoding_rs::ISO_8859_15.decode(bytes).0.to_string())
        }
        "windows-1252" | "cp1252" => {
            Ok(encoding_rs::WINDOWS_1252.decode(bytes).0.to_string())
        }
        _ => {
            // Fallback: try UTF-8 with lossy conversion
            Ok(String::from_utf8_lossy(bytes).to_string())
        }
    }
    .map_err(|e: std::string::FromUtf8Error| CsvError::new(0, format!("Encoding error: {}", e)))
}

/// Detect the delimiter by counting occurrences in the first line
pub fn detect_delimiter(content: &str) -> char {
    let first_line = content.lines().next().unwrap_or("");
    
    let separators = [';', ',', '\t', '|'];
    let mut best_sep = ';';
    let mut best_count = 0;
    
    for &sep in &separators {
        let count = first_line.matches(sep).count();
        if count > best_count {
            best_count = count;
            best_sep = sep;
        }
    }
    
    best_sep
}

/// Parse CSV into JSON objects with explicit delimiter.
///
/// Each row becomes a JSON object where keys are column headers.
///
/// # Example
/// ```ignore
/// use massload::csv_to_json;
///
/// let csv = "name;age\nAlice;30\nBob;25";
/// let rows = csv_to_json(csv, ';').unwrap();
///
/// assert_eq!(rows.len(), 2);
/// assert_eq!(rows[0]["name"], "Alice");
/// assert_eq!(rows[0]["age"], "30");
/// ```
pub fn csv_to_json(csv: &str, delimiter: char) -> Result<Vec<Value>, CsvError> {
    parse_csv(csv.as_bytes(), delimiter)
}

/// Parse CSV from a reader into JSON objects.
pub fn parse_csv<R: Read>(reader: R, delimiter: char) -> Result<Vec<Value>, CsvError> {
    let buf = BufReader::new(reader);
    let mut lines = buf.lines();

    // Get headers from first line
    let header_line = lines.next()
        .ok_or_else(|| CsvError::new(1, "Empty CSV file"))?
        .map_err(|e| CsvError::new(1, format!("Cannot read header: {}", e)))?;

    let headers: Vec<String> = header_line
        .split(delimiter)
        .map(|s| s.trim().trim_matches('"').to_string())
        .collect();

    if headers.is_empty() {
        return Err(CsvError::new(1, "No headers found"));
    }

    // Parse data rows
    let mut rows = Vec::new();

    for (line_idx, line_result) in lines.enumerate() {
        let line_num = line_idx + 2; // +1 for 0-index, +1 for header

        let line = line_result
            .map_err(|e| CsvError::new(line_num, format!("Cannot read line: {}", e)))?;
        
        if line.trim().is_empty() {
            continue;
        }

        let values: Vec<&str> = line.split(delimiter).collect();
        let mut obj = Map::new();

        for (i, header) in headers.iter().enumerate() {
            let raw_value = values.get(i)
                .map(|s| s.trim().trim_matches('"'))
                .unwrap_or("");
            
            obj.insert(header.clone(), json!(raw_value));
        }

        rows.push(Value::Object(obj));
    }

    Ok(rows)
}

/// Parse CSV file with auto-detection of encoding and delimiter.
///
/// # Example
/// ```ignore
/// let result = parse_csv_file_auto("/path/to/file.csv")?;
/// println!("Encoding: {}, Delimiter: '{}'", result.encoding, result.delimiter);
/// println!("Records: {}", result.records.len());
/// ```
pub fn parse_csv_file_auto<P: AsRef<Path>>(path: P) -> Result<ParseResult, CsvError> {
    let bytes = std::fs::read(path.as_ref())
        .map_err(|e| CsvError::new(0, format!("Cannot read file: {}", e)))?;
    
    parse_bytes_auto(&bytes)
}

/// Parse CSV bytes with auto-detection of encoding and delimiter.
pub fn parse_bytes_auto(bytes: &[u8]) -> Result<ParseResult, CsvError> {
    // Detect encoding
    let encoding = detect_encoding(bytes);
    
    // Decode content
    let content = decode_content(bytes, &encoding)?;
    
    // Detect delimiter
    let delimiter = detect_delimiter(&content);
    
    // Parse with detected settings
    parse_string_with_metadata(&content, delimiter, encoding)
}

/// Parse CSV string with explicit delimiter and return metadata.
pub fn parse_string_with_metadata(content: &str, delimiter: char, encoding: String) -> Result<ParseResult, CsvError> {
    let mut lines = content.lines();

    // Get headers from first line
    let header_line = lines.next()
        .ok_or_else(|| CsvError::new(1, "Empty CSV file"))?;

    let headers: Vec<String> = header_line
        .split(delimiter)
        .map(|s| s.trim().trim_matches('"').to_string())
        .collect();

    if headers.is_empty() {
        return Err(CsvError::new(1, "No headers found"));
    }

    // Parse data rows
    let mut records = Vec::new();

    for line in lines {
        
        if line.trim().is_empty() {
            continue;
        }

        let values: Vec<&str> = line.split(delimiter).collect();
        let mut obj = Map::new();

        for (i, header) in headers.iter().enumerate() {
            let raw_value = values.get(i)
                .map(|s| s.trim().trim_matches('"'))
                .unwrap_or("");
            
            obj.insert(header.clone(), json!(raw_value));
        }

        records.push(Value::Object(obj));
    }

    Ok(ParseResult {
        records,
        encoding,
        delimiter,
        headers,
    })
}

/// Parse CSV file with explicit delimiter.
pub fn parse_csv_file(path: &str, delimiter: char) -> Result<Vec<Value>, CsvError> {
    let file = std::fs::File::open(path)
        .map_err(|e| CsvError::new(0, format!("Cannot open file '{}': {}", path, e)))?;
    parse_csv(file, delimiter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_csv() {
        let csv = "name;age\nAlice;30\nBob;25";
        let rows = csv_to_json(csv, ';').unwrap();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["name"], "Alice");
        assert_eq!(rows[0]["age"], "30");
        assert_eq!(rows[1]["name"], "Bob");
        assert_eq!(rows[1]["age"], "25");
    }

    #[test]
    fn test_comma_delimiter() {
        let csv = "a,b,c\n1,2,3";
        let rows = csv_to_json(csv, ',').unwrap();

        assert_eq!(rows[0]["a"], "1");
        assert_eq!(rows[0]["b"], "2");
        assert_eq!(rows[0]["c"], "3");
    }

    #[test]
    fn test_quoted_values() {
        let csv = r#"name;value
"Alice";"Hello World""#;
        let rows = csv_to_json(csv, ';').unwrap();

        assert_eq!(rows[0]["name"], "Alice");
        assert_eq!(rows[0]["value"], "Hello World");
    }

    #[test]
    fn test_empty_lines_skipped() {
        let csv = "a;b\n1;2\n\n3;4\n";
        let rows = csv_to_json(csv, ';').unwrap();

        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_missing_values() {
        let csv = "a;b;c\n1;;3";
        let rows = csv_to_json(csv, ';').unwrap();

        assert_eq!(rows[0]["a"], "1");
        assert_eq!(rows[0]["b"], "");
        assert_eq!(rows[0]["c"], "3");
    }

    #[test]
    fn test_extra_columns_ignored() {
        let csv = "a;b\n1;2;3;4";
        let rows = csv_to_json(csv, ';').unwrap();

        assert_eq!(rows[0]["a"], "1");
        assert_eq!(rows[0]["b"], "2");
    }

    #[test]
    fn test_error_message_format() {
        let err = CsvError::new(5, "Invalid value")
            .with_column("age")
            .with_value("abc");
        
        let msg = err.to_string();
        assert!(msg.contains("Line 5"));
        assert!(msg.contains("column 'age'"));
        assert!(msg.contains("value 'abc'"));
    }

    #[test]
    fn test_empty_csv_error() {
        let result = csv_to_json("", ';');
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Empty"));
    }

    #[test]
    fn test_detect_delimiter_semicolon() {
        let content = "a;b;c\n1;2;3";
        assert_eq!(detect_delimiter(content), ';');
    }

    #[test]
    fn test_detect_delimiter_comma() {
        let content = "a,b,c\n1,2,3";
        assert_eq!(detect_delimiter(content), ',');
    }

    #[test]
    fn test_detect_delimiter_tab() {
        let content = "a\tb\tc\n1\t2\t3";
        assert_eq!(detect_delimiter(content), '\t');
    }

    #[test]
    fn test_detect_delimiter_pipe() {
        let content = "a|b|c\n1|2|3";
        assert_eq!(detect_delimiter(content), '|');
    }

    #[test]
    fn test_auto_parse() {
        let csv = "name;age\nAlice;30\nBob;25";
        let result = parse_bytes_auto(csv.as_bytes()).unwrap();
        
        assert_eq!(result.delimiter, ';');
        assert_eq!(result.records.len(), 2);
        assert_eq!(result.headers, vec!["name", "age"]);
    }

    #[test]
    fn test_latin1_decoding() {
        // "Société" in ISO-8859-1
        let bytes: &[u8] = &[0x53, 0x6F, 0x63, 0x69, 0xE9, 0x74, 0xE9];
        let decoded = decode_content(bytes, "iso-8859-1").unwrap();
        assert!(decoded.contains("Soci"));
    }
}
