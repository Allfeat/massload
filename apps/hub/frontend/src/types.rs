//! Common types used across the frontend application.
//!
//! This module centralizes type definitions to avoid duplication
//! and ensure consistency across components.
//!
//! # Categories
//!
//! - **Preview Types** - UI display types
//! - **Log Types** - Real-time log streaming
//! - **API Types** - Backend response structures
//! - **Wallet Types** - Blockchain wallet info
//! - **Error Types** - Frontend error handling

use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// Preview Types
// =============================================================================

/// Summary item for the preview list.
///
/// Represents a single musical work in the preview UI.
#[derive(Clone, Debug, PartialEq)]
pub struct PreviewItem {
    /// Work title
    pub title: String,
    /// ISWC identifier
    pub iswc: String,
    /// Number of creators
    pub creators_count: usize,
}

// =============================================================================
// Log Types
// =============================================================================

/// Log severity level.
///
/// Matches the backend's log levels for SSE streaming.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    /// Informational message
    Info,
    /// Success/completion message
    Success,
    /// Error message
    Error,
    /// Warning message
    Warning,
    /// Debug message (verbose)
    Debug,
}

impl LogLevel {
    /// Get CSS class for styling.
    pub fn css_class(&self) -> &'static str {
        match self {
            LogLevel::Info => "log-info",
            LogLevel::Success => "log-success",
            LogLevel::Error => "log-error",
            LogLevel::Warning => "log-warning",
            LogLevel::Debug => "log-debug",
        }
    }

    /// Get emoji prefix for display.
    pub fn emoji(&self) -> &'static str {
        match self {
            LogLevel::Info => "ℹ️",
            LogLevel::Success => "✅",
            LogLevel::Error => "❌",
            LogLevel::Warning => "⚠️",
            LogLevel::Debug => "🔍",
        }
    }
}

/// A single log entry from the backend.
///
/// Received via SSE from `/api/logs` endpoint.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    /// Severity level
    pub level: LogLevel,
    /// Log message
    pub message: String,
    /// Timestamp string (HH:MM:SS)
    pub timestamp: String,
}

// =============================================================================
// API Response Types
// =============================================================================

/// Response from the backend upload endpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    /// Unique job identifier
    pub job_id: String,
    /// Status: "ready", "warning", "error"
    pub status: String,
    /// Musical works in MIDDS format
    pub musical_works: Vec<serde_json::Value>,
    /// Metadata about the transformation
    pub metadata: ResponseMetadata,
}

/// Metadata about the transformation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    /// Total number of works
    pub total_works: usize,
    /// Estimated cost in AFT
    pub estimated_cost: String,
    /// Template ID used (if cached)
    pub matrix_id: Option<String>,
    /// Whether a cached template was used
    pub cached: bool,
}

// =============================================================================
// Wallet Types
// =============================================================================

/// Connected wallet information.
#[derive(Clone, Debug, PartialEq)]
pub struct WalletInfo {
    /// SS58 encoded address
    pub address: String,
    /// Display name (from extension)
    pub name: Option<String>,
    /// Wallet extension name
    pub source: String,
}

// =============================================================================
// Error Types
// =============================================================================

/// Frontend application errors.
///
/// Unified error type for all frontend operations.
#[derive(Clone, Debug)]
pub enum AppError {
    /// File upload failed.
    Upload(String),
    /// Wallet connection failed.
    Wallet(String),
    /// Blockchain transaction failed.
    Blockchain(String),
    /// Network/HTTP error.
    Network(String),
    /// Invalid data format.
    Validation(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Upload(msg) => write!(f, "Upload error: {}", msg),
            AppError::Wallet(msg) => write!(f, "Wallet error: {}", msg),
            AppError::Blockchain(msg) => write!(f, "Blockchain error: {}", msg),
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// Result type alias for frontend operations.
pub type AppResult<T> = Result<T, AppError>;

