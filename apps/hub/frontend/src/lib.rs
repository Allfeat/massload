//! Allfeat Apps Hub - Frontend Rust/Leptos Application
//!
//! A WebAssembly frontend for the Allfeat ecosystem apps:
//! - Mass Load: Bulk CSV registration
//! - Register: Single work registration (coming soon)
//! - Protect: IP protection (coming soon)
//!
//! # Architecture (Tanssi-style)
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────┐
//! │  Sidebar │              Main Content                         │
//! │  ├─Home  │  ┌─────────────────────────────────────────────┐  │
//! │  ├─Apps  │  │  Header (wallet connection)                 │  │
//! │  │ ├─ML  │  ├─────────────────────────────────────────────┤  │
//! │  │ ├─Reg │  │  Page Content (routed)                      │  │
//! │  │ └─Pro │  │  - HomePage / MassloadPage / etc.           │  │
//! │  └─Links │  ├─────────────────────────────────────────────┤  │
//! │          │  │  Footer                                      │  │
//! └──────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Modules
//!
//! - [`types`] - Common types (LogEntry, PreviewItem, etc.)
//! - [`components`] - UI components (Sidebar, Header, etc.)
//! - [`pages`] - Route pages (Home, Massload, Register, Protect)
//! - [`services`] - Backend communication (upload, wallet, blockchain)

use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

// =============================================================================
// Module declarations
// =============================================================================

pub mod config;
pub mod i18n;
pub mod types;
pub mod components;
pub mod pages;
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
    
    log::info!("🚀 Allfeat Apps Hub - Starting");
    
    // Mount the application
    mount_to_body(|| view! { <App/> });
}

#[component]
pub fn App() -> impl IntoView {
    // Initialize language context
    i18n::provide_language_context();
    
    // Global wallet state (shared across all pages)
    let (wallet_connected, set_wallet_connected) = create_signal(false);
    let (wallet_address, set_wallet_address) = create_signal(None::<String>);
    
    view! {
        <Router>
            <div class="app-layout">
                // Left sidebar navigation
                <components::Sidebar/>
                
                // Main content area
                <div class="main-area">
                    // Top header with wallet
                    <Header 
                        wallet_connected=wallet_connected 
                        wallet_address=wallet_address
                        set_wallet_connected=set_wallet_connected
                        set_wallet_address=set_wallet_address
                    />
                    
                    // Page content (routed)
                    <main class="page-content">
                        <Routes>
                            <Route path="/" view=pages::HomePage/>
                            <Route path="/massload" view=move || view! {
                                <pages::MassloadPage 
                                    wallet_connected=wallet_connected
                                    wallet_address=wallet_address
                                />
                            }/>
                            <Route path="/register" view=pages::RegisterPage/>
                            <Route path="/protect" view=pages::ProtectPage/>
                        </Routes>
                    </main>
                    
                    // Footer
                    <Footer/>
                </div>
            </div>
        </Router>
    }
}
