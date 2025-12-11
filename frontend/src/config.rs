//! Application configuration.
//!
//! Configuration is loaded at RUNTIME from window.MASSLOAD_CONFIG (set by config.js).
//! This allows the same Docker image to be used in different environments.
//!
//! # Unified Server Architecture
//!
//! The backend serves the frontend, so API calls use relative paths (no hostname).
//! Only external services (like blockchain RPC) need absolute URLs.

use wasm_bindgen::prelude::*;

// =============================================================================
// Runtime configuration getters
// =============================================================================

/// Get a config value from window.MASSLOAD_CONFIG
fn get_config_value(key: &str) -> Option<String> {
    let window = web_sys::window()?;
    let config = js_sys::Reflect::get(&window, &JsValue::from_str("MASSLOAD_CONFIG")).ok()?;
    if config.is_undefined() || config.is_null() {
        return None;
    }
    let value = js_sys::Reflect::get(&config, &JsValue::from_str(key)).ok()?;
    value.as_string()
}

/// Get the blockchain RPC endpoint.
/// 
/// Reads from `window.MASSLOAD_CONFIG.BLOCKCHAIN_RPC` at runtime.
/// Falls back to devnet for development.
pub fn blockchain_rpc() -> String {
    get_config_value("BLOCKCHAIN_RPC")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            log::warn!("BLOCKCHAIN_RPC not set in config.js, using default");
            "wss://node-dev.allfeat.io".to_string()
        })
}

// =============================================================================
// Compile-time constants (not environment-dependent)
// =============================================================================

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
