//! Application configuration.
//!
//! Centralized configuration for the Mass Load frontend.
//! Variables can be set at build time via environment variables.
//!
//! # Build-time configuration
//! ```bash
//! BACKEND_URL=https://api.example.com trunk build --release
//! ```

/// Backend API base URL.
///
/// Set via `BACKEND_URL` env var at build time.
/// Defaults to `http://localhost:3000` for development.
pub const BACKEND_URL: &str = match option_env!("BACKEND_URL") {
    Some(url) => url,
    None => "http://localhost:3000",
};

/// Blockchain RPC endpoint.
///
/// Set via `BLOCKCHAIN_RPC` env var at build time.
/// Defaults to devnet for development.
pub const BLOCKCHAIN_RPC: &str = match option_env!("BLOCKCHAIN_RPC") {
    Some(url) => url,
    None => "wss://node-dev.allfeat.io",
};

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

