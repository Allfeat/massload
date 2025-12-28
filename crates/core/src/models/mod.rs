//! MIDDS (Music Industry Decentralized Data Structures) domain models.
//!
//! This module contains the core data structures used throughout the Allfeat ecosystem:
//!
//! - [`PartyId`] - IPI or ISNI identifier for a party (creator/publisher)
//! - [`Creator`] - Creator information with ID and role
//! - [`MusicalWork`] - Complete MIDDS musical work
//! - [`WorkType`] - Type of work (Original, Medley, etc.)
//! - [`ClassicalInfo`] - Classical music specific information
//!
//! ## Feature Flags
//!
//! - `wasm`: WASM-compatible types for frontend (removes backend-only fields)
//! - `validation`: Includes JSON schema validation
//!
//! ## Usage
//!
//! ```rust
//! use allfeat_core::models::{MusicalWork, Creator, PartyId};
//!
//! let mut work = MusicalWork::new("T1234567890", "My Song");
//! work.add_creator(Creator::new(PartyId::Ipi(123456789), "Composer"));
//! work.validate()?;
//! ```

pub mod party;
pub mod creator;
pub mod musical_work;

// Re-export main types
pub use party::PartyId;
pub use creator::{Creator, CreatorRole};
#[cfg(not(target_arch = "wasm32"))]
pub use creator::CisacRole;
pub use musical_work::{MusicalWork, WorkType, ClassicalInfo};

// Legacy alias for backward compatibility with backend
#[cfg(not(target_arch = "wasm32"))]
pub use musical_work::MusicalWork as GroupedWork;

// Legacy alias for backward compatibility
#[cfg(not(target_arch = "wasm32"))]
pub use musical_work::WorkType as MusicalWorkType;

