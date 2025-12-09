//! Header component with Allfeat branding and theme toggle

use leptos::*;
use crate::services::wallet::PolkadotWallet;
use crate::services::blockchain::get_wallet_balance;

/// Allfeat logo SVG (light version - for dark backgrounds)
fn logo_light() -> &'static str {
    r##"<svg viewBox="0 0 512 124.5" xmlns="http://www.w3.org/2000/svg"><g fill="#fffbeb"><path d="m294.26,124.5c-25.88,0-44.12-17.2-44.12-45.16s18.06-45.16,43.25-45.16,40.99,16.15,40.99,45.16v5.38h-63.05v.17c0,14.24,9.03,23.62,22.93,23.62,10.77,0,17.02-5.56,18.93-12.51h19.98c-3.3,16.85-17.72,28.49-38.91,28.49Zm-22.93-53.84h42.38c0-12.33-8.16-20.5-20.67-20.5s-20.67,7.64-21.71,20.5Z"/><path d="m371.66,124.5c-17.54,0-26.75-10.25-26.75-23.27,0-14.94,11.81-22.93,25.19-26.4l28.66-7.12v-1.56c0-11.64-6.08-15.98-15.98-15.98-10.6,0-16.5,5.21-16.85,15.81h-20.67c1.39-21.02,17.2-31.79,38.39-31.79,22.93,0,35.61,13.03,35.61,34.39v54.19h-20.15v-14.42c-5.73,10.25-14.42,16.15-27.44,16.15Zm-6.25-25.88c0,7.47,5.38,11.12,13.55,11.12,11.64,0,19.8-7.82,19.8-20.5v-7.99l-20.67,5.04c-8.69,2.08-12.68,6.08-12.68,12.33Z"/><path d="m460.55,123.7c-16.15,0-22.06-7.3-22.06-21.89v-47.59h-10.94v-17.37h10.94V15.83l20.5-3.3v24.32h16.33v17.37h-16.33v44.81c0,5.91,1.56,7.3,7.29,7.3h9.03v17.37h-14.76Z"/><path d="m211.97,22.23c0-13.72,6.43-22.23,21.54-22.23h13.9v16.85h-7.82c-5.38,0-7.12,1.56-7.12,7.3v12.51h14.94v17.37h-14.94v69.48h-20.5V54.02h-11.12v-17.37h11.12v-14.42Z"/><path d="m161.55,0h20.5v99.35c0,5.73,1.74,7.3,7.12,7.3h9.55v16.85h-15.63c-15.11,0-21.54-8.51-21.54-22.23V0Z"/><path d="m117.33,0h20.5v99.35c0,5.73,1.74,7.3,7.12,7.3h9.55v16.85h-15.63c-15.11,0-21.54-8.51-21.54-22.23V0Z"/><path d="m74.83,13.93l36.16,109.77h-21.54l-10.42-32.48H31.79l-10.25,32.48H0L36.33,13.91C39.07,5.6,46.83,0,55.57,0h0c8.75,0,16.52,5.62,19.26,13.93Zm-37.14,58.53h35.43l-16.81-53.16c-.27-.89-1.54-.89-1.81,0l-16.81,53.16Z"/><path d="m499.44,123.7c-6.94,0-12.56-5.62-12.56-12.56h0c0-6.94,5.62-12.56,12.56-12.56h0c6.94,0,12.56,5.62,12.56,12.56h0c0,6.94-5.62,12.56-12.56,12.56h0Z"/></g></svg>"##
}

/// Allfeat logo SVG (dark version - for light backgrounds)
fn logo_dark() -> &'static str {
    r##"<svg viewBox="0 0 512 124.5" xmlns="http://www.w3.org/2000/svg"><g fill="#151515"><path d="m294.26,124.5c-25.88,0-44.12-17.2-44.12-45.16s18.06-45.16,43.25-45.16,40.99,16.15,40.99,45.16v5.38h-63.05v.17c0,14.24,9.03,23.62,22.93,23.62,10.77,0,17.02-5.56,18.93-12.51h19.98c-3.3,16.85-17.72,28.49-38.91,28.49Zm-22.93-53.84h42.38c0-12.33-8.16-20.5-20.67-20.5s-20.67,7.64-21.71,20.5Z"/><path d="m371.66,124.5c-17.54,0-26.75-10.25-26.75-23.27,0-14.94,11.81-22.93,25.19-26.4l28.66-7.12v-1.56c0-11.64-6.08-15.98-15.98-15.98-10.6,0-16.5,5.21-16.85,15.81h-20.67c1.39-21.02,17.2-31.79,38.39-31.79,22.93,0,35.61,13.03,35.61,34.39v54.19h-20.15v-14.42c-5.73,10.25-14.42,16.15-27.44,16.15Zm-6.25-25.88c0,7.47,5.38,11.12,13.55,11.12,11.64,0,19.8-7.82,19.8-20.5v-7.99l-20.67,5.04c-8.69,2.08-12.68,6.08-12.68,12.33Z"/><path d="m460.55,123.7c-16.15,0-22.06-7.3-22.06-21.89v-47.59h-10.94v-17.37h10.94V15.83l20.5-3.3v24.32h16.33v17.37h-16.33v44.81c0,5.91,1.56,7.3,7.29,7.3h9.03v17.37h-14.76Z"/><path d="m211.97,22.23c0-13.72,6.43-22.23,21.54-22.23h13.9v16.85h-7.82c-5.38,0-7.12,1.56-7.12,7.3v12.51h14.94v17.37h-14.94v69.48h-20.5V54.02h-11.12v-17.37h11.12v-14.42Z"/><path d="m161.55,0h20.5v99.35c0,5.73,1.74,7.3,7.12,7.3h9.55v16.85h-15.63c-15.11,0-21.54-8.51-21.54-22.23V0Z"/><path d="m117.33,0h20.5v99.35c0,5.73,1.74,7.3,7.12,7.3h9.55v16.85h-15.63c-15.11,0-21.54-8.51-21.54-22.23V0Z"/><path d="m74.83,13.93l36.16,109.77h-21.54l-10.42-32.48H31.79l-10.25,32.48H0L36.33,13.91C39.07,5.6,46.83,0,55.57,0h0c8.75,0,16.52,5.62,19.26,13.93Zm-37.14,58.53h35.43l-16.81-53.16c-.27-.89-1.54-.89-1.81,0l-16.81,53.16Z"/><path d="m499.44,123.7c-6.94,0-12.56-5.62-12.56-12.56h0c0-6.94,5.62-12.56,12.56-12.56h0c6.94,0,12.56,5.62,12.56,12.56h0c0,6.94-5.62,12.56-12.56,12.56h0Z"/></g></svg>"##
}

#[component]
pub fn Header(
    wallet_connected: ReadSignal<bool>,
    wallet_address: ReadSignal<Option<String>>,
    set_wallet_connected: WriteSignal<bool>,
    set_wallet_address: WriteSignal<Option<String>>,
) -> impl IntoView {
    // Balance state
    let (balance, set_balance) = create_signal(None::<String>);
    // Theme state (true = dark, false = light)
    let (is_dark, set_is_dark) = create_signal(true);
    
    // Toggle theme
    let toggle_theme = move |_| {
        let new_theme = !is_dark.get();
        set_is_dark.set(new_theme);
        
        // Update DOM
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let theme = if new_theme { "" } else { "light" };
                    let _ = html.set_attribute("data-theme", theme);
                }
            }
        }
    };
    
    // Handler pour connexion wallet
    let on_wallet_click = move |_| {
        if !wallet_connected.get() {
            log::info!("🔑 Attempting to connect wallet...");
            
            spawn_local(async move {
                match PolkadotWallet::connect().await {
                    Ok(account) => {
                        log::info!("✅ Wallet connected: {}", account.address);
                        set_wallet_connected.set(true);
                        set_wallet_address.set(Some(account.address.clone()));
                        
                        // Fetch balance
                        match get_wallet_balance(&account.address).await {
                            Ok(bal) => {
                                log::info!("💰 Balance: {} MEL", bal.formatted);
                                set_balance.set(Some(bal.formatted));
                            }
                            Err(e) => {
                                log::warn!("Could not fetch balance: {}", e);
                                set_balance.set(Some("?".to_string()));
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("❌ Wallet connection failed: {}", e);
                    }
                }
            });
        }
    };

    view! {
        <header>
            <div class="header-left">
                <a href="/" class="logo" inner_html=move || {
                    if is_dark.get() { logo_light() } else { logo_dark() }
                }></a>
                <span class="badge">
                    {move || {
                        if let Some(bal) = balance.get() {
                            format!("{} MEL", bal)
                        } else {
                            "-- MEL".to_string()
                        }
                    }}
                </span>
            </div>
            <div class="header-right">
                // Theme toggle
                <div class="theme-toggle" on:click=toggle_theme>
                    <span class="theme-icon" class:active=move || !is_dark.get()>"☀️"</span>
                    <div class="theme-switch"></div>
                    <span class="theme-icon" class:active=move || is_dark.get()>"🌙"</span>
                </div>
                
                // Wallet connection
                <div 
                    class="wallet-status" 
                    class:connected=move || wallet_connected.get()
                    on:click=on_wallet_click
                >
                    <svg class="wallet-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <rect x="2" y="6" width="20" height="14" rx="2"/>
                        <path d="M16 14h.01"/>
                        <path d="M2 10h20"/>
                    </svg>
                    <span>
                        {move || if let Some(addr) = wallet_address.get() {
                            format!("{}...{}", &addr[0..6.min(addr.len())], &addr[addr.len().saturating_sub(4)..])
                        } else {
                            "Connecter wallet".to_string()
                        }}
                    </span>
                </div>
            </div>
        </header>
    }
}
