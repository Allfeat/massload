//! Party identification types (IPI, ISNI).

use serde::{Deserialize, Serialize};

/// Unique identifier for a party (creator/publisher).
///
/// Can be either an IPI (Interested Party Identifier) or ISNI
/// (International Standard Name Identifier), or both.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value")]
pub enum PartyId {
    /// IPI number (9-11 digits).
    /// Using u32 for WASM compatibility (frontend needs u32 for blockchain SDK).
    Ipi(u32),
    
    /// ISNI (16 characters).
    Isni(String),
    
    /// Both IPI and ISNI (for backend CSV processing).
    #[cfg(not(target_arch = "wasm32"))]
    Both { ipi: u32, isni: String },
}

impl PartyId {
    /// Create a PartyId from optional IPI and ISNI values.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_optional(ipi: Option<u32>, isni: Option<String>) -> Option<Self> {
        match (ipi, isni) {
            (Some(i), Some(s)) => Some(PartyId::Both { ipi: i, isni: s }),
            (Some(i), None) => Some(PartyId::Ipi(i)),
            (None, Some(s)) => Some(PartyId::Isni(s)),
            (None, None) => None,
        }
    }

    /// Get the IPI if present.
    pub fn ipi(&self) -> Option<u32> {
        match self {
            PartyId::Ipi(i) => Some(*i),
            #[cfg(not(target_arch = "wasm32"))]
            PartyId::Both { ipi, .. } => Some(*ipi),
            PartyId::Isni(_) => None,
        }
    }

    /// Get the ISNI if present.
    pub fn isni(&self) -> Option<&str> {
        match self {
            PartyId::Isni(s) => Some(s),
            #[cfg(not(target_arch = "wasm32"))]
            PartyId::Both { isni, .. } => Some(isni),
            PartyId::Ipi(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_party_id_ipi() {
        let party = PartyId::Ipi(123456789);
        assert_eq!(party.ipi(), Some(123456789));
        assert_eq!(party.isni(), None);
    }

    #[test]
    fn test_party_id_isni() {
        let party = PartyId::Isni("0000000123456789".into());
        assert_eq!(party.ipi(), None);
        assert_eq!(party.isni(), Some("0000000123456789"));
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
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
}

