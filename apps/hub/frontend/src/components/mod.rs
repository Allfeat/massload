//! UI Components for the Allfeat Apps Hub.
//!
//! This module contains all Leptos components organized by function:
//!
//! # Layout Components
//! - [`Sidebar`] - Left navigation menu (Tanssi-style)
//! - [`Header`] - Top bar with wallet connection
//! - [`Footer`] - Page footer
//!
//! # Page Components
//! - [`Hero`] - Main title and description
//! - [`UploadSection`] - CSV file upload with drag & drop
//! - [`PreviewSection`] - Transaction preview before submission
//! - [`LogsPanel`] - Real-time processing logs (SSE)
//! - [`ProgressSection`] - Transaction progress indicator

mod sidebar;
mod header;
mod hero;
mod upload;
mod preview;
pub mod midds_display; // Public module with submodules
mod progress;
mod footer;
mod logs;
mod wallet_modal;
pub mod icons;
pub mod musical_work_form_v2;

pub use sidebar::*;
pub use header::*;
pub use hero::*;
pub use upload::*;
pub use preview::*;
// Re-export MIDDS display components
pub use midds_display::{MiddsDisplay, WorkDisplay, WorkDisplayJson, RecordingDisplay, ReleaseDisplay};
pub use progress::*;
pub use footer::*;
pub use logs::*;
pub use wallet_modal::*;
pub use icons::*;

