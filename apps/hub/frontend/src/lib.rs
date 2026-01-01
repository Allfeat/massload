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
pub mod network;
pub mod types;
pub mod components;
pub mod pages;
pub mod services;
pub mod validation;
pub mod midds;

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

// Explorer types
pub use services::explorer::{
    MusicalWorkData, RecordingData, ReleaseData,
    CreatorData, PerformerData,
    MiddsItem,
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
    
    // Initialize theme context (from allfeat-ui)
    allfeat_ui::provide_theme_context();
    
    // Initialize network context (blockchain RPC selector)
    network::provide_network_context();
    
    // Global wallet state (shared across all pages)
    let (wallet_connected, set_wallet_connected) = create_signal(false);
    let (wallet_address, set_wallet_address) = create_signal(None::<String>);
    
    // Provide wallet signals to context so child components can use them
    provide_context(wallet_connected);
    provide_context(wallet_address);
    
    view! {
        <Router>
            <div class="app-layout">
                // Top header (full width, like massload)
                <Header 
                    wallet_connected=wallet_connected 
                    wallet_address=wallet_address
                    set_wallet_connected=set_wallet_connected
                    set_wallet_address=set_wallet_address
                />
                
                // Content area with sidebar
                <div class="content-with-sidebar">
                    // Left sidebar navigation (below header)
                    <components::Sidebar/>
                    
                    // Main content area
                    <div class="main-area">
                        // Page content (routed)
                        <main class="page-content">
                            <Routes>
                                // Home
                                <Route path="/" view=pages::HomePage/>
                                
                                // Explore - View MIDDS on-chain
                                <Route path="/explore" view=pages::ExplorePage/>
                                
                                // Register - direct to forms
                                <Route path="/register" view=pages::RegisterPage/>
                                <Route path="/register-musical-work" view=pages::RegisterMusicalWorkPage/>
                                <Route path="/register-recording" view=pages::RegisterRecordingPage/>
                                <Route path="/register-release" view=pages::RegisterReleasePage/>
                                <Route path="/register-artist" view=pages::RegisterArtistPage/>
                                <Route path="/register-legal-entity" view=pages::RegisterLegalEntityPage/>
                                
                                // Protect
                                <Route path="/protect" view=pages::ProtectPage/>
                                
                                // Mass Load
                                <Route path="/massload" view=move || view! {
                                    <pages::MassloadPage 
                                        wallet_connected=wallet_connected
                                        wallet_address=wallet_address
                                        set_wallet_connected=set_wallet_connected
                                        set_wallet_address=set_wallet_address
                                    />
                                }/>
                                
                                // How it works
                                <Route path="/how-it-works" view=pages::HowItWorksPage/>
                            </Routes>
                        </main>
                        
                        // Footer
                        <Footer/>
                    </div>
                </div>
            </div>
        </Router>
    }
}
