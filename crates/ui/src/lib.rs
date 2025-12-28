//! # Allfeat UI
//!
//! Shared UI components for the Allfeat ecosystem.
//!
//! This crate provides reusable Leptos components that can be used across
//! multiple Allfeat applications (Hub, Register, Protect, etc.).
//!
//! ## Modules
//!
//! - [`components`] - Reusable UI components (Header, Footer, Sidebar, Icons)
//! - [`state`] - State management (Wallet, Notifications)
//! - [`utils`] - UI utilities (i18n, theme, etc.)

pub mod components;
pub mod state;
pub mod utils;

// Re-export commonly used items
pub use components::*;
