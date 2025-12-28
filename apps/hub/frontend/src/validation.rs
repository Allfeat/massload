//! Form validation utilities based on MIDDS JSON Schema

use regex::Regex;

/// ISWC validation (T + 10 digits, format: T1234567890)
pub fn validate_iswc(iswc: &str) -> Result<(), String> {
    if iswc.is_empty() {
        return Err("ISWC is required".to_string());
    }
    
    if iswc.len() != 11 {
        return Err("ISWC must be exactly 11 characters".to_string());
    }
    
    let re = Regex::new(r"^T\d{10}$").unwrap();
    if !re.is_match(iswc) {
        return Err("ISWC format must be T followed by 10 digits (e.g., T1234567890)".to_string());
    }
    
    Ok(())
}

/// Title validation (1-256 characters)
pub fn validate_title(title: &str) -> Result<(), String> {
    if title.is_empty() {
        return Err("Title is required".to_string());
    }
    
    if title.len() > 256 {
        return Err("Title must not exceed 256 characters".to_string());
    }
    
    Ok(())
}

/// Creation year validation (1000-9999)
pub fn validate_year(year: &str) -> Result<(), String> {
    if year.is_empty() {
        return Ok(()); // Optional field
    }
    
    match year.parse::<i32>() {
        Ok(y) if (1000..=9999).contains(&y) => Ok(()),
        Ok(_) => Err("Year must be between 1000 and 9999".to_string()),
        Err(_) => Err("Invalid year format".to_string()),
    }
}

/// BPM validation (1-65535)
pub fn validate_bpm(bpm: &str) -> Result<(), String> {
    if bpm.is_empty() {
        return Ok(()); // Optional field
    }
    
    match bpm.parse::<u16>() {
        Ok(b) if b >= 1 => Ok(()),
        Ok(_) => Err("BPM must be at least 1".to_string()),
        Err(_) => Err("Invalid BPM format".to_string()),
    }
}

/// IPI validation (9 digits)
pub fn validate_ipi(ipi: &str) -> Result<(), String> {
    if ipi.is_empty() {
        return Ok(()); // Can be empty if ISNI is provided
    }
    
    let re = Regex::new(r"^\d{9}$").unwrap();
    if !re.is_match(ipi) {
        return Err("IPI must be exactly 9 digits".to_string());
    }
    
    Ok(())
}

/// ISNI validation (16 digits)
pub fn validate_isni(isni: &str) -> Result<(), String> {
    if isni.is_empty() {
        return Ok(()); // Can be empty if IPI is provided
    }
    
    let re = Regex::new(r"^\d{16}$").unwrap();
    if !re.is_match(isni) {
        return Err("ISNI must be exactly 16 digits".to_string());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_iswc() {
        assert!(validate_iswc("T1234567890").is_ok());
        assert!(validate_iswc("T0000000000").is_ok());
        assert!(validate_iswc("").is_err());
        assert!(validate_iswc("1234567890").is_err());
        assert!(validate_iswc("T123456789").is_err());
        assert!(validate_iswc("T12345678901").is_err());
    }
    
    #[test]
    fn test_validate_title() {
        assert!(validate_title("Valid Title").is_ok());
        assert!(validate_title("A").is_ok());
        assert!(validate_title("").is_err());
        assert!(validate_title(&"A".repeat(256)).is_ok());
        assert!(validate_title(&"A".repeat(257)).is_err());
    }
    
    #[test]
    fn test_validate_year() {
        assert!(validate_year("").is_ok()); // Optional
        assert!(validate_year("2024").is_ok());
        assert!(validate_year("1000").is_ok());
        assert!(validate_year("9999").is_ok());
        assert!(validate_year("999").is_err());
        assert!(validate_year("10000").is_err());
    }
}

