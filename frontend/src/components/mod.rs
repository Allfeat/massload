//! UI Components for the Mass Load application.
//!
//! This module contains all Leptos components organized by function:
//!
//! # Layout Components
//! - [`Header`] - Navigation bar with wallet connection
//! - [`Hero`] - Main title and description
//! - [`Footer`] - Page footer
//!
//! # Feature Components
//! - [`UploadSection`] - CSV file upload with drag & drop
//! - [`PreviewSection`] - Transaction preview before submission
//! - `PreviewDetail` - Detailed view of a single musical work
//! - [`LogsPanel`] - Real-time processing logs (SSE)
//! - [`ProgressSection`] - Transaction progress indicator

mod header;
mod hero;
mod upload;
mod preview;
mod preview_detail;
mod progress;
mod footer;
mod logs;

pub use header::*;
pub use hero::*;
pub use upload::*;
pub use preview::*;
pub use preview_detail::*;
pub use progress::*;
pub use footer::*;
pub use logs::*;

