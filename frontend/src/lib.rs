//! Mass Load - Frontend Rust/Leptos Application
//!
//! A WebAssembly frontend for uploading CSV files and registering
//! musical works on the Allfeat blockchain.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                        App                                   │
//! ├─────────────────────────────────────────────────────────────┤
//! │  Header (wallet connection)                                  │
//! ├─────────────────────────────────────────────────────────────┤
//! │  MainContent                                                 │
//! │  ├── Hero (title, description)                              │
//! │  ├── UploadSection or LogsPanel                             │
//! │  └── PreviewSection (when works loaded)                     │
//! ├─────────────────────────────────────────────────────────────┤
//! │  Footer                                                      │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Modules
//!
//! - [`types`] - Common types (LogEntry, PreviewItem, etc.)
//! - [`components`] - UI components (Header, Upload, Preview, etc.)
//! - [`services`] - Backend communication (upload, wallet, blockchain)

use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

// =============================================================================
// Module declarations
// =============================================================================

pub mod config;
pub mod types;
pub mod components;
pub mod services;

// =============================================================================
// Re-exports
// =============================================================================

// Configuration
pub use config::*;

// Types
pub use types::{
    // Preview
    PreviewItem,
    // Logs
    LogEntry, LogLevel,
    // API
    UploadResponse, ResponseMetadata,
    // Wallet
    WalletInfo,
    // Errors
    AppError, AppResult,
};

// Components
pub use components::*;

// Services
pub use services::*;

// =============================================================================
// Application Entry Point
// =============================================================================

/// WASM entry point - called automatically by trunk.
#[wasm_bindgen(start)]
pub fn main() {
    // Setup panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Setup console logging
    _ = console_log::init_with_level(log::Level::Debug);
    
    log::info!("🦀 Mass Load Rust - Starting Leptos App");
    
    // Mount the application
    mount_to_body(|| view! { <App/> });
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=MainContent/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn MainContent() -> impl IntoView {
    // Global state for the application
    let (wallet_connected, set_wallet_connected) = create_signal(false);
    let (wallet_address, set_wallet_address) = create_signal(None::<String>);
    let (preview_data, set_preview_data) = create_signal(None::<Vec<PreviewItem>>);
    let (musical_works_json, set_musical_works_json) = create_signal(None::<serde_json::Value>);
    let (_is_processing, set_is_processing) = create_signal(false);
    let (logs, set_logs) = create_signal(Vec::<LogEntry>::new());
    
    // Initialize SSE connection ONCE at app startup
    init_sse_logs(set_logs);

    view! {
        <Header 
            wallet_connected=wallet_connected 
            wallet_address=wallet_address
            set_wallet_connected=set_wallet_connected
            set_wallet_address=set_wallet_address
        />

        <div class="container">
            <Hero/>

            // Show UploadBox when no logs, hide when logs exist
            <Show
                when=move || logs.get().is_empty()
                fallback=|| view! { }
            >
                <UploadSection 
                    set_preview_data=set_preview_data
                    set_musical_works_json=set_musical_works_json
                    set_is_processing=set_is_processing 
                    set_logs=set_logs
                />
            </Show>
            
            // Show LogsPanel when logs exist
            <Show
                when=move || !logs.get().is_empty()
                fallback=|| view! { }
            >
                <LogsPanel logs=logs set_logs=set_logs/>
            </Show>

            // Preview section (appears after processing)
            <Show
                when=move || preview_data.get().is_some()
                fallback=|| view! { }
            >
                <PreviewSection 
                    data=preview_data
                    musical_works_json=musical_works_json
                    wallet_connected=wallet_connected
                    wallet_address=wallet_address
                    set_logs=set_logs
                    set_is_processing=set_is_processing
                    set_preview_data=set_preview_data
                    set_musical_works_json=set_musical_works_json
                />
            </Show>
        </div>

        <Footer/>
    }
}
