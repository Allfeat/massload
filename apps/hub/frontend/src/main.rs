//! Entry point for the WASM application

use allfeat_hub_frontend::App;
use leptos::*;

pub fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    log::info!("🚀 Allfeat Apps Hub - Starting");
    
    mount_to_body(|| view! { <App/> })
}
