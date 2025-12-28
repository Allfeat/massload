//! MIDDS (Music Industry Decentralized Data Structures) types for frontend.
//!
//! This module re-exports types from `allfeat-core` for use in the frontend.
//! All types are now defined centrally in `crates/core/src/models/`.

// Re-export all MIDDS types from core
pub use allfeat_core::models::{
    PartyId,
    Creator,
    MusicalWork,
    WorkType,
    ClassicalInfo,
};

// Note: Validation is done in the form component and by the blockchain runtime.
// Frontend only performs basic format checks before submission.
