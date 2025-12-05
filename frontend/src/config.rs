//! Application configuration.
//!
//! Centralized configuration for the Mass Load frontend.
//! In development, these are hardcoded. In production, they could be
//! loaded from environment or a config file.

/// Backend API base URL.
///
/// The massload backend server for CSV transformation.
pub const BACKEND_URL: &str = "http://localhost:3000";

/// Blockchain RPC endpoint.
///
/// WebSocket URL for the Allfeat node.
pub const BLOCKCHAIN_RPC: &str = "wss://node-dev.allfeat.io";

/// Application name for wallet connection.
///
/// Displayed in wallet extension popups.
pub const APP_NAME: &str = "Mass Load";

/// Maximum file size for upload (in bytes).
///
/// 50 MB limit.
pub const MAX_FILE_SIZE: usize = 50 * 1024 * 1024;

/// Maximum logs to keep in memory.
pub const MAX_LOG_ENTRIES: usize = 100;

/// Cost per musical work in AFT (estimated).
pub const COST_PER_WORK: f64 = 0.05;

