//! Creator and role types.

use serde::{Deserialize, Serialize};
use super::party::PartyId;

/// Role of a creator in a musical work.
///
/// For frontend/blockchain: uses simple String role.
/// For backend CSV parsing: uses typed enum with CISAC codes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), serde(untagged))]
pub enum CreatorRole {
    /// Freeform role string (used by frontend/blockchain).
    String(String),
    
    /// Typed CISAC role (used by backend CSV parsing).
    #[cfg(not(target_arch = "wasm32"))]
    Typed(CisacRole),
}

/// CISAC role codes (for backend CSV processing).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CisacRole {
    /// Composer (C) - Music creator
    Composer,
    /// Author (A) - Lyrics writer
    Author,
    /// Lyricist (synonym for Author)
    Lyricist,
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

#[cfg(not(target_arch = "wasm32"))]
impl CisacRole {
    /// Parse role from CISAC code string.
    pub fn from_code(code: &str) -> Option<Self> {
        let normalized = code.trim().to_uppercase();
        match normalized.as_str() {
            "C" | "COMPOSER" | "COMPOSITEUR" => Some(Self::Composer),
            "A" | "AUTHOR" | "AUTEUR" => Some(Self::Author),
            "LYRICIST" | "PAROLIER" => Some(Self::Lyricist),
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
            Self::Author | Self::Lyricist => "A",
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

impl CreatorRole {
    /// Create from string (for frontend).
    pub fn from_string(s: impl Into<String>) -> Self {
        Self::String(s.into())
    }
    
    /// Get as string (for serialization).
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(s) => s,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Typed(role) => match role {
                CisacRole::Composer => "Composer",
                CisacRole::Author => "Author",
                CisacRole::Lyricist => "Lyricist",
                CisacRole::ComposerAuthor => "Composer/Author",
                CisacRole::Arranger => "Arranger",
                CisacRole::Adapter => "Adapter",
                CisacRole::Translator => "Translator",
                CisacRole::SubAuthor => "SubAuthor",
                CisacRole::SubArranger => "SubArranger",
                CisacRole::Publisher => "Publisher",
                CisacRole::OriginalPublisher => "OriginalPublisher",
                CisacRole::SubPublisher => "SubPublisher",
            },
        }
    }
}

/// A creator (interested party) of a musical work.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    /// Unique identifier (IPI, ISNI, or both).
    pub id: PartyId,
    
    /// Role in the work (String for blockchain, enum for backend).
    pub role: String,
    
    /// Display name (optional, for UI/backend only).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(not(target_arch = "wasm32"))]
    pub name: Option<String>,
    
    /// Share percentage (0-100, for backend only).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg(not(target_arch = "wasm32"))]
    pub share: Option<f64>,
}

impl Creator {
    /// Create a simple creator (for frontend/blockchain).
    pub fn new(id: PartyId, role: impl Into<String>) -> Self {
        Self {
            id,
            role: role.into(),
            #[cfg(not(target_arch = "wasm32"))]
            name: None,
            #[cfg(not(target_arch = "wasm32"))]
            share: None,
        }
    }
    
    /// Create a full creator (for backend).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_full(id: PartyId, role: String, name: Option<String>, share: Option<f64>) -> Self {
        Self {
            id,
            role,
            name,
            share,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creator_new() {
        let creator = Creator::new(PartyId::Ipi(123456789), "Composer");
        assert_eq!(creator.role, "Composer");
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_cisac_role_from_code() {
        assert_eq!(CisacRole::from_code("C"), Some(CisacRole::Composer));
        assert_eq!(CisacRole::from_code("AUTEUR"), Some(CisacRole::Author));
        assert_eq!(CisacRole::from_code("ca"), Some(CisacRole::ComposerAuthor));
        assert_eq!(CisacRole::from_code("INVALID"), None);
    }
}

