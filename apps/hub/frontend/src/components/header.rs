//! Header component with Allfeat branding and theme toggle
//! Styled to match register.allfeat.org exactly
//!
//! Now uses ThemeState from allfeat-ui for global theme management

use leptos::*;
use allfeat_ui::{use_theme, Theme, IconGlobe, IconSun, IconMoon, IconWallet};
use crate::services::wallet::PolkadotWallet;
use crate::services::blockchain::get_wallet_balance;
use crate::i18n::{Language, t, use_language, use_set_language};
use crate::components::wallet_modal::{WalletModal, WalletType};

/// Light theme logo path (dark text, for light backgrounds)
const LOGO_DARK_PATH: &str = "/public/logo-dark.png";

/// Dark theme logo path (light text, for dark backgrounds)
const LOGO_LIGHT_PATH: &str = "/public/logo-light.png";

#[component]
pub fn Header(
    wallet_connected: ReadSignal<bool>,
    wallet_address: ReadSignal<Option<String>>,
    set_wallet_connected: WriteSignal<bool>,
    set_wallet_address: WriteSignal<Option<String>>,
) -> impl IntoView {
    // Get language context
    let lang = use_language();
    let set_lang = use_set_language();
    
    // Get global theme state from allfeat-ui
    let theme_state = use_theme();
    
    // Balance state
    let (balance, set_balance) = create_signal(None::<String>);
    // Language dropdown state
    let (lang_dropdown_open, set_lang_dropdown_open) = create_signal(false);
    // Wallet modal state
    let (wallet_modal_open, set_wallet_modal_open) = create_signal(false);
    
    // Toggle theme (now uses ThemeState)
    let toggle_theme = move |_| {
        theme_state.toggle_and_save();
    };
    
    // Toggle language dropdown
    let toggle_lang_dropdown = move |_| {
        set_lang_dropdown_open.update(|open| *open = !*open);
    };
    
    // Select language
    let select_language = move |new_lang: Language| {
        set_lang.set(new_lang);
        set_lang_dropdown_open.set(false);
    };
    
    // Handler pour ouvrir la modal de wallet
    let on_wallet_click = move |_| {
        if !wallet_connected.get() {
            set_wallet_modal_open.set(true);
        }
    };

    // Handler pour la sélection d'un wallet
    let on_wallet_select = Callback::new(move |wallet_type: WalletType| {
        set_wallet_modal_open.set(false);
        
        let wallet_key = match wallet_type {
            WalletType::SubWallet => "subwallet-js",
            WalletType::Talisman => "talisman",
            WalletType::PolkadotJs => "polkadot-js",
        };
        
        log::info!("🔑 Attempting to connect {}...", wallet_key);
        
        spawn_local(async move {
            match PolkadotWallet::connect_specific(wallet_key).await {
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
    });

    // Handler pour fermer la modal
    let on_wallet_modal_close = Callback::new(move |_| {
        set_wallet_modal_open.set(false);
    });

    view! {
        <>
        <header>
            <div class="header-left">
                <a href="/" class="logo">
                    <img 
                        src=move || if theme_state.get() == Theme::Dark { LOGO_LIGHT_PATH } else { LOGO_DARK_PATH }
                        alt="Allfeat Logo"
                        class="logo-img"
                    />
                </a>
            </div>
            <div class="header-right">
                // MEL balance badge
                <span class="badge">
                    {move || {
                        if let Some(bal) = balance.get() {
                            format!("{} MEL", bal)
                        } else {
                            "-- MEL".to_string()
                        }
                    }}
                </span>
                
                // Language selector with dropdown
                <div class="lang-selector-container">
                    <button 
                        class="lang-selector"
                        on:click=toggle_lang_dropdown
                    >
                        <span class="lang-icon"><IconGlobe /></span>
                        <span class="lang-flag">{move || lang.get().flag()}</span>
                        <span class="lang-text">{move || lang.get().name()}</span>
                    </button>
                    
                    // Dropdown menu
                    <Show when=move || lang_dropdown_open.get()>
                        <div class="lang-dropdown">
                            {Language::all().iter().map(|&l| {
                                view! {
                                    <button 
                                        class="lang-option"
                                        class:active=move || lang.get() == l
                                        on:click=move |_| select_language(l)
                                    >
                                        <span class="lang-flag">{l.flag()}</span>
                                        <span>{l.name()}</span>
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    </Show>
                </div>
                
                // Theme toggle
                <div class="theme-toggle" on:click=toggle_theme>
                    <span 
                        class="theme-icon" 
                        class:icon-active=move || theme_state.get() == Theme::Light
                    >
                        <IconSun />
                    </span>
                    <div class="theme-switch" class:checked=move || theme_state.get() == Theme::Dark></div>
                    <span 
                        class="theme-icon" 
                        class:icon-active=move || theme_state.get() == Theme::Dark
                    >
                        <IconMoon />
                    </span>
                </div>
                
                // Wallet connection button
                <div 
                    class="wallet-status" 
                    class:connected=move || wallet_connected.get()
                    on:click=on_wallet_click
                >
                    <span class="wallet-icon"><IconWallet /></span>
                    <span>
                        {move || if let Some(addr) = wallet_address.get() {
                            format!("{}...{}", &addr[0..6.min(addr.len())], &addr[addr.len().saturating_sub(4)..])
                        } else {
                            t("header.connect_wallet")
                        }}
                    </span>
                </div>
            </div>
        </header>

        // Wallet selection modal
        <WalletModal
            show=Signal::derive(move || wallet_modal_open.get())
            on_select=on_wallet_select
            on_close=on_wallet_modal_close
        />
        </>
    }
}
