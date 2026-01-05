//! Excel to CSV conversion module.
//!
//! Detects and converts Excel files (.xlsx, .xls) to CSV format
//! for processing by the transformation pipeline.

use calamine::{open_workbook_from_rs, Reader, Xlsx, Xls, DataType, CellType};
use std::io::Cursor;

/// Supported Excel file types
#[derive(Debug, Clone, Copy)]
pub enum ExcelType {
    Xlsx,
    Xls,
}

/// Detect if file content is Excel based on magic bytes
pub fn detect_excel_type(bytes: &[u8]) -> Option<ExcelType> {
    // Check for XLSX (ZIP signature: PK)
    if bytes.len() >= 4 && &bytes[0..4] == b"PK\x03\x04" {
        return Some(ExcelType::Xlsx);
    }
    
    // Check for XLS (OLE2 signature: D0 CF 11 E0 A1 B1 1A E1)
    if bytes.len() >= 8 && &bytes[0..8] == b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1" {
        return Some(ExcelType::Xls);
    }
    
    None
}

/// Convert Excel bytes to CSV string
pub fn excel_to_csv(bytes: &[u8], excel_type: ExcelType) -> Result<String, String> {
    match excel_type {
        ExcelType::Xlsx => xlsx_to_csv(bytes),
        ExcelType::Xls => xls_to_csv(bytes),
    }
}

/// Convert XLSX to CSV
fn xlsx_to_csv(bytes: &[u8]) -> Result<String, String> {
    let cursor = Cursor::new(bytes);
    let mut workbook: Xlsx<_> = open_workbook_from_rs(cursor)
        .map_err(|e| format!("Failed to open XLSX: {}", e))?;
    
    // Get the first worksheet
    let sheet_names = workbook.sheet_names();
    if sheet_names.is_empty() {
        return Err("No worksheets found in Excel file".to_string());
    }
    
    let first_sheet = &sheet_names[0];
    let range = workbook
        .worksheet_range(first_sheet)
        .map_err(|e| format!("Failed to read worksheet: {}", e))?;
    
    // Convert to CSV
    range_to_csv(&range)
}

/// Convert XLS to CSV
fn xls_to_csv(bytes: &[u8]) -> Result<String, String> {
    let cursor = Cursor::new(bytes);
    let mut workbook: Xls<_> = open_workbook_from_rs(cursor)
        .map_err(|e| format!("Failed to open XLS: {}", e))?;
    
    // Get the first worksheet
    let sheet_names = workbook.sheet_names();
    if sheet_names.is_empty() {
        return Err("No worksheets found in Excel file".to_string());
    }
    
    let first_sheet = &sheet_names[0];
    let range = workbook
        .worksheet_range(first_sheet)
        .map_err(|e| format!("Failed to read worksheet: {}", e))?;
    
    // Convert to CSV
    range_to_csv(&range)
}

/// Convert a calamine Range to CSV string
fn range_to_csv<T>(range: &calamine::Range<T>) -> Result<String, String>
where
    T: CellType + DataType + std::fmt::Display,
{
    let mut csv = String::new();
    
    for row in range.rows() {
        let row_str: Vec<String> = row
            .iter()
            .map(|cell| cell_to_string(cell))
            .collect();
        
        csv.push_str(&row_str.join(","));
        csv.push('\n');
    }
    
    Ok(csv)
}

/// Convert a DataType cell to a properly escaped CSV string
fn cell_to_string<T: DataType + std::fmt::Display>(cell: &T) -> String {
    // Convert cell to string and escape if necessary
    let value = cell.to_string();
    
    // Empty cells
    if value.is_empty() {
        return String::new();
    }
    
    escape_csv_field(&value)
}

/// Escape a field for CSV (handle commas, quotes, newlines)
fn escape_csv_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        // Escape quotes by doubling them
        let escaped = s.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_escape_csv_field() {
        assert_eq!(escape_csv_field("simple"), "simple");
        assert_eq!(escape_csv_field("has, comma"), "\"has, comma\"");
        assert_eq!(escape_csv_field("has \"quote\""), "\"has \"\"quote\"\"\"");
    }
}

