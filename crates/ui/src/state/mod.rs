//! State management for UI components.
//!
//! This module contains:
//! - Wallet state (connection, address, balance)
//! - Notification state (toasts/alerts)
//! - Theme state (dark/light mode)

pub mod wallet;
pub mod theme;
pub mod notification;

// Re-export commonly used items
pub use wallet::*;
pub use theme::*;
pub use notification::*;

