//! Musical work types.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::creator::Creator;

/// Type of musical work.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum WorkType {
    /// Original composition.
    #[default]
    Original,
    /// Medley of multiple works.
    Medley,
    /// Mashup of existing works.
    Mashup,
    /// Adaptation/arrangement of existing work.
    Adaptation,
}

/// Classical music specific information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClassicalInfo {
    /// Opus number (e.g., "Op. 27 No. 2").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opus: Option<String>,
    
    /// Catalog number (e.g., "BWV 1007").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_number: Option<String>,
    
    /// Number of voices/parts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_voices: Option<u32>,
}

/// A complete musical work in MIDDS format.
///
/// This structure is compatible with both:
/// - Frontend/blockchain submission (minimal fields)
/// - Backend CSV processing (extended fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicalWork {
    // ===== Required fields =====
    
    /// ISWC (International Standard Musical Work Code).
    /// Format: T followed by 10 digits (e.g., "T1234567890").
    pub iswc: String,
    
    /// Main title of the work.
    pub title: String,
    
    /// All creators of the work (composers, lyricists, etc.).
    pub creators: Vec<Creator>,
    
    // ===== Optional metadata =====
    
    /// Alternative titles.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    #[cfg(not(target_arch = "wasm32"))]
    pub alternative_titles: Vec<String>,
    
    /// Year of creation (4 digits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_year: Option<u32>,
    
    /// Whether the work is instrumental (no lyrics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrumental: Option<bool>,
    
    /// Language of lyrics (if not instrumental).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    
    /// Beats per minute (tempo).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bpm: Option<u16>,
    
    /// Musical key (e.g., "Am", "C", "F#m").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    
    /// Type of work (Original, Medley, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_type: Option<WorkType>,
    
    /// Classical music information (opus, catalog, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classical_info: Option<ClassicalInfo>,
    
    /// Musical genre (for backend only).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(not(target_arch = "wasm32"))]
    pub genre: Option<String>,
    
    // ===== Participants (reserved for future use) =====
    
    /// Participants (performers, etc.) - currently empty.
    /// Reserved for future MIDDS extensions.
    #[serde(default)]
    pub participants: Vec<Value>,
}

impl MusicalWork {
    /// Create a new work with minimal required fields.
    pub fn new(iswc: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            iswc: iswc.into(),
            title: title.into(),
            creators: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            alternative_titles: Vec::new(),
            creation_year: None,
            instrumental: None,
            language: None,
            bpm: None,
            key: None,
            work_type: None,
            classical_info: None,
            #[cfg(not(target_arch = "wasm32"))]
            genre: None,
            participants: Vec::new(),
        }
    }

    /// Add a creator to the work.
    pub fn add_creator(&mut self, creator: Creator) {
        self.creators.push(creator);
    }
    
    /// Basic validation (frontend).
    ///
    /// Full JSON schema validation will be done by backend.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = vec![];
        
        // Validate ISWC format
        if !self.iswc.starts_with('T') || self.iswc.len() != 11 {
            errors.push("ISWC must be T followed by 10 digits (e.g., T1234567890)".to_string());
        }
        
        // Validate title
        if self.title.is_empty() {
            errors.push("Title is required".to_string());
        } else if self.title.len() > 256 {
            errors.push("Title must not exceed 256 characters".to_string());
        }
        
        // Validate creators
        if self.creators.is_empty() {
            errors.push("At least one creator is required".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PartyId;

    #[test]
    fn test_musical_work_new() {
        let work = MusicalWork::new("T1234567890", "Test Song");
        assert_eq!(work.iswc, "T1234567890");
        assert_eq!(work.title, "Test Song");
        assert!(work.creators.is_empty());
    }

    #[test]
    fn test_musical_work_validation_valid() {
        let mut work = MusicalWork::new("T1234567890", "Test Song");
        work.add_creator(Creator::new(PartyId::Ipi(123456789), "Composer"));
        
        assert!(work.validate().is_ok());
    }

    #[test]
    fn test_musical_work_validation_invalid_iswc() {
        let work = MusicalWork::new("INVALID", "Test Song");
        let result = work.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("ISWC")));
    }

    #[test]
    fn test_musical_work_validation_no_creators() {
        let work = MusicalWork::new("T1234567890", "Test Song");
        let result = work.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("creator")));
    }
    
    #[test]
    fn test_work_type_serialization() {
        let work_type = WorkType::Original;
        let json = serde_json::to_string(&work_type).unwrap();
        assert_eq!(json, "\"Original\"");
    }
}

