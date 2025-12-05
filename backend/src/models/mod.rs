//! Domain models for the Massload transformation pipeline.
//!
//! This module contains the core data structures used throughout the pipeline:
//!
//! - [`GroupedWork`] - Complete MIDDS musical work with all creators
//! - [`Creator`] - Creator information with ID and role
//! - [`PartyId`] - IPI or ISNI identifier for a creator
//! - [`CreatorRole`] - CISAC role codes (Composer, Author, etc.)
//! - [`MusicalWorkType`] - Type of work (Original, Arrangement, etc.)

use serde::{Deserialize, Serialize};

// =============================================================================
// Party Identification
// =============================================================================

/// Unique identifier for a party (creator/publisher).
///
/// Can be either an IPI (Interested Party Identifier) or ISNI
/// (International Standard Name Identifier), or both.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum PartyId {
    /// IPI number (9-11 digits).
    Ipi(u64),
    /// ISNI (16 characters).
    Isni(String),
    /// Both IPI and ISNI.
    Both { ipi: u64, isni: String },
}

impl PartyId {
    /// Create a PartyId from optional IPI and ISNI values.
    pub fn from_optional(ipi: Option<u64>, isni: Option<String>) -> Option<Self> {
        match (ipi, isni) {
            (Some(i), Some(s)) => Some(PartyId::Both { ipi: i, isni: s }),
            (Some(i), None) => Some(PartyId::Ipi(i)),
            (None, Some(s)) => Some(PartyId::Isni(s)),
            (None, None) => None,
        }
    }

    /// Get the IPI if present.
    pub fn ipi(&self) -> Option<u64> {
        match self {
            PartyId::Ipi(i) => Some(*i),
            PartyId::Both { ipi, .. } => Some(*ipi),
            PartyId::Isni(_) => None,
        }
    }

    /// Get the ISNI if present.
    pub fn isni(&self) -> Option<&str> {
        match self {
            PartyId::Isni(s) => Some(s),
            PartyId::Both { isni, .. } => Some(isni),
            PartyId::Ipi(_) => None,
        }
    }
}

// =============================================================================
// Creator Role
// =============================================================================

/// Role of a creator in a musical work.
///
/// Based on CISAC role codes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CreatorRole {
    /// Composer (C) - Music creator
    Composer,
    /// Author (A) - Lyrics writer
    Author,
    /// Composer and Author (CA)
    ComposerAuthor,
    /// Arranger (AR)
    Arranger,
    /// Adapter (AD)
    Adapter,
    /// Translator (TR)
    Translator,
    /// Sub-Author (SA)
    SubAuthor,
    /// Sub-Arranger (SR)
    SubArranger,
    /// Publisher (E)
    Publisher,
    /// Original Publisher (SE)
    OriginalPublisher,
    /// Sub-Publisher (ES)
    SubPublisher,
}

impl CreatorRole {
    /// Parse role from CISAC code string.
    pub fn from_code(code: &str) -> Option<Self> {
        let normalized = code.trim().to_uppercase();
        match normalized.as_str() {
            "C" | "COMPOSER" | "COMPOSITEUR" => Some(Self::Composer),
            "A" | "AUTHOR" | "AUTEUR" | "LYRICIST" | "PAROLIER" => Some(Self::Author),
            "CA" | "COMPOSER_AUTHOR" | "AUTEUR-COMPOSITEUR" => Some(Self::ComposerAuthor),
            "AR" | "ARRANGER" | "ARRANGEUR" => Some(Self::Arranger),
            "AD" | "ADAPTER" | "ADAPTATEUR" => Some(Self::Adapter),
            "TR" | "TRANSLATOR" | "TRADUCTEUR" => Some(Self::Translator),
            "SA" | "SUB_AUTHOR" | "SOUS-AUTEUR" => Some(Self::SubAuthor),
            "SR" | "SUB_ARRANGER" | "SOUS-ARRANGEUR" => Some(Self::SubArranger),
            "E" | "PUBLISHER" | "EDITEUR" => Some(Self::Publisher),
            "SE" | "ORIGINAL_PUBLISHER" | "EDITEUR_ORIGINAL" => Some(Self::OriginalPublisher),
            "ES" | "SUB_PUBLISHER" | "SOUS-EDITEUR" => Some(Self::SubPublisher),
            _ => None,
        }
    }

    /// Convert to CISAC code.
    pub fn to_code(&self) -> &'static str {
        match self {
            Self::Composer => "C",
            Self::Author => "A",
            Self::ComposerAuthor => "CA",
            Self::Arranger => "AR",
            Self::Adapter => "AD",
            Self::Translator => "TR",
            Self::SubAuthor => "SA",
            Self::SubArranger => "SR",
            Self::Publisher => "E",
            Self::OriginalPublisher => "SE",
            Self::SubPublisher => "ES",
        }
    }
}

// =============================================================================
// Creator
// =============================================================================

/// A creator (interested party) of a musical work.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    /// Unique identifier (IPI, ISNI, or both).
    pub id: PartyId,
    /// Role in the work.
    pub role: CreatorRole,
    /// Display name (optional, for UI).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Share percentage (0-100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<f64>,
}

// =============================================================================
// Musical Work Type
// =============================================================================

/// Type of musical work.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(tag = "type")]
pub enum MusicalWorkType {
    /// Original composition.
    #[default]
    Original,
    /// Arrangement of existing work.
    Arrangement,
    /// Composite work (medley, etc.).
    Composite,
    /// Excerpt from larger work.
    Excerpt,
    /// Unspecified work type.
    Unspecified,
}

// =============================================================================
// Grouped Musical Work (MIDDS format)
// =============================================================================

/// A complete musical work in MIDDS format.
///
/// This is the final output format, with all creators grouped together.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupedWork {
    /// ISWC (International Standard Musical Work Code).
    pub iswc: String,
    /// Main title of the work.
    pub title: String,
    /// Alternative titles.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub alternative_titles: Vec<String>,
    /// All creators of the work.
    pub creators: Vec<Creator>,
    /// Participants (performers, etc.) - currently empty, reserved for future use.
    #[serde(default)]
    pub participants: Vec<serde_json::Value>,
    /// Type of work.
    #[serde(default)]
    pub work_type: MusicalWorkType,
    /// Year of creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_year: Option<u16>,
    /// Musical genre.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    /// Whether the work is instrumental (no lyrics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrumental: Option<bool>,
}

impl GroupedWork {
    /// Create a new work with minimal required fields.
    pub fn new(iswc: String, title: String) -> Self {
        Self {
            iswc,
            title,
            alternative_titles: Vec::new(),
            creators: Vec::new(),
            participants: Vec::new(),
            work_type: MusicalWorkType::Original,
            creation_year: None,
            genre: None,
            instrumental: None,
        }
    }

    /// Add a creator to the work.
    pub fn add_creator(&mut self, creator: Creator) {
        self.creators.push(creator);
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_party_id_from_optional() {
        assert!(matches!(
            PartyId::from_optional(Some(123), None),
            Some(PartyId::Ipi(123))
        ));
        assert!(matches!(
            PartyId::from_optional(None, Some("0000000123456789".into())),
            Some(PartyId::Isni(_))
        ));
        assert!(matches!(
            PartyId::from_optional(Some(123), Some("0000000123456789".into())),
            Some(PartyId::Both { .. })
        ));
        assert!(PartyId::from_optional(None, None).is_none());
    }

    #[test]
    fn test_creator_role_from_code() {
        assert_eq!(CreatorRole::from_code("C"), Some(CreatorRole::Composer));
        assert_eq!(CreatorRole::from_code("AUTEUR"), Some(CreatorRole::Author));
        assert_eq!(CreatorRole::from_code("ca"), Some(CreatorRole::ComposerAuthor));
        assert_eq!(CreatorRole::from_code("INVALID"), None);
    }

    #[test]
    fn test_creator_role_roundtrip() {
        let role = CreatorRole::Composer;
        let code = role.to_code();
        assert_eq!(CreatorRole::from_code(code), Some(CreatorRole::Composer));
    }

    #[test]
    fn test_grouped_work_serialization() {
        let work = GroupedWork::new("T1234567890".into(), "Test Song".into());
        let json = serde_json::to_string(&work).unwrap();
        assert!(json.contains("T1234567890"));
        assert!(json.contains("Test Song"));
    }
}

