//! Entry point for the WASM application

use frontend_rust::App;
use leptos::*;

pub fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    log::info!("🦀 Mass Load Rust - Starting Leptos App");
    
    mount_to_body(|| view! { <App/> })
}
