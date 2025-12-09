//! Header component with Allfeat branding and theme toggle
//! Styled to match register.allfeat.org exactly

use leptos::*;
use crate::services::wallet::PolkadotWallet;
use crate::services::blockchain::get_wallet_balance;
use crate::i18n::{Language, Translations, use_language, use_set_language};
use crate::components::wallet_modal::{WalletModal, WalletType};

/// Globe icon SVG for language selector
fn globe_icon() -> &'static str {
    r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="2" y1="12" x2="22" y2="12"></line><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path></svg>"##
}

/// Sun icon SVG (from radix-ui, same as register.allfeat.org)
fn sun_icon() -> &'static str {
    r##"<svg width="18" height="18" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M7.5 0C7.77614 0 8 0.223858 8 0.5V2.5C8 2.77614 7.77614 3 7.5 3C7.22386 3 7 2.77614 7 2.5V0.5C7 0.223858 7.22386 0 7.5 0ZM2.1967 2.1967C2.39196 2.00144 2.70854 2.00144 2.90381 2.1967L4.31802 3.61091C4.51328 3.80617 4.51328 4.12276 4.31802 4.31802C4.12276 4.51328 3.80617 4.51328 3.61091 4.31802L2.1967 2.90381C2.00144 2.70854 2.00144 2.39196 2.1967 2.1967ZM0.5 7C0.223858 7 0 7.22386 0 7.5C0 7.77614 0.223858 8 0.5 8H2.5C2.77614 8 3 7.77614 3 7.5C3 7.22386 2.77614 7 2.5 7H0.5ZM2.1967 12.8033C2.00144 12.608 2.00144 12.2915 2.1967 12.0962L3.61091 10.682C3.80617 10.4867 4.12276 10.4867 4.31802 10.682C4.51328 10.8772 4.51328 11.1938 4.31802 11.3891L2.90381 12.8033C2.70854 12.9986 2.39196 12.9986 2.1967 12.8033ZM12.5 7C12.2239 7 12 7.22386 12 7.5C12 7.77614 12.2239 8 12.5 8H14.5C14.7761 8 15 7.77614 15 7.5C15 7.22386 14.7761 7 14.5 7H12.5ZM10.682 4.31802C10.4867 4.12276 10.4867 3.80617 10.682 3.61091L12.0962 2.1967C12.2915 2.00144 12.608 2.00144 12.8033 2.1967C12.9986 2.39196 12.9986 2.70854 12.8033 2.90381L11.3891 4.31802C11.1938 4.51328 10.8772 4.51328 10.682 4.31802ZM8 12.5C8 12.2239 7.77614 12 7.5 12C7.22386 12 7 12.2239 7 12.5V14.5C7 14.7761 7.22386 15 7.5 15C7.77614 15 8 14.7761 8 14.5V12.5ZM10.682 10.682C10.8772 10.4867 11.1938 10.4867 11.3891 10.682L12.8033 12.0962C12.9986 12.2915 12.9986 12.608 12.8033 12.8033C12.608 12.9986 12.2915 12.9986 12.0962 12.8033L10.682 11.3891C10.4867 11.1938 10.4867 10.8772 10.682 10.682ZM5.5 7.5C5.5 6.39543 6.39543 5.5 7.5 5.5C8.60457 5.5 9.5 6.39543 9.5 7.5C9.5 8.60457 8.60457 9.5 7.5 9.5C6.39543 9.5 5.5 8.60457 5.5 7.5ZM7.5 4.5C5.84315 4.5 4.5 5.84315 4.5 7.5C4.5 9.15685 5.84315 10.5 7.5 10.5C9.15685 10.5 10.5 9.15685 10.5 7.5C10.5 5.84315 9.15685 4.5 7.5 4.5Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>"##
}

/// Moon icon SVG (from radix-ui, same as register.allfeat.org)
fn moon_icon() -> &'static str {
    r##"<svg width="18" height="18" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M2.89998 0.499976C2.89998 0.279062 2.72089 0.0999756 2.49998 0.0999756C2.27906 0.0999756 2.09998 0.279062 2.09998 0.499976V1.09998H1.49998C1.27906 1.09998 1.09998 1.27906 1.09998 1.49998C1.09998 1.72089 1.27906 1.89998 1.49998 1.89998H2.09998V2.49998C2.09998 2.72089 2.27906 2.89998 2.49998 2.89998C2.72089 2.89998 2.89998 2.72089 2.89998 2.49998V1.89998H3.49998C3.72089 1.89998 3.89998 1.72089 3.89998 1.49998C3.89998 1.27906 3.72089 1.09998 3.49998 1.09998H2.89998V0.499976ZM5.89998 3.49998C5.89998 3.27906 5.72089 3.09998 5.49998 3.09998C5.27906 3.09998 5.09998 3.27906 5.09998 3.49998V4.09998H4.49998C4.27906 4.09998 4.09998 4.27906 4.09998 4.49998C4.09998 4.72089 4.27906 4.89998 4.49998 4.89998H5.09998V5.49998C5.09998 5.72089 5.27906 5.89998 5.49998 5.89998C5.72089 5.89998 5.89998 5.72089 5.89998 5.49998V4.89998H6.49998C6.72089 4.89998 6.89998 4.72089 6.89998 4.49998C6.89998 4.27906 6.72089 4.09998 6.49998 4.09998H5.89998V3.49998ZM1.89998 6.49998C1.89998 6.27906 1.72089 6.09998 1.49998 6.09998C1.27906 6.09998 1.09998 6.27906 1.09998 6.49998V7.09998H0.499976C0.279062 7.09998 0.0999756 7.27906 0.0999756 7.49998C0.0999756 7.72089 0.279062 7.89998 0.499976 7.89998H1.09998V8.49998C1.09998 8.72089 1.27906 8.89997 1.49998 8.89997C1.72089 8.89997 1.89998 8.72089 1.89998 8.49998V7.89998H2.49998C2.72089 7.89998 2.89998 7.72089 2.89998 7.49998C2.89998 7.27906 2.72089 7.09998 2.49998 7.09998H1.89998V6.49998ZM8.54406 0.98184L8.24618 0.941586C8.03275 0.917676 7.90692 1.1655 8.02936 1.34194C8.17013 1.54479 8.29981 1.75592 8.41754 1.97445C8.91878 2.90485 9.20322 3.96932 9.20322 5.10022C9.20322 8.37201 6.82247 11.0878 3.69887 11.6097C3.45736 11.65 3.20988 11.6772 2.96008 11.6906C2.74563 11.702 2.62729 11.9535 2.77721 12.1072C2.84551 12.1773 2.91535 12.2458 2.98667 12.3128L3.05883 12.3795L3.31883 12.6045L3.50684 12.7532L3.62796 12.8433L3.81491 12.9742L3.99079 13.089C4.11175 13.1651 4.23536 13.2375 4.36157 13.3059L4.62496 13.4412L4.88553 13.5607L5.18837 13.6828L5.43169 13.7686C5.56564 13.8128 5.70149 13.8529 5.83857 13.8885C5.94262 13.9155 6.04767 13.9401 6.15405 13.9622C6.27993 13.9883 6.40713 14.0109 6.53544 14.0298L6.85241 14.0685L7.11934 14.0892C7.24637 14.0965 7.37436 14.1002 7.50322 14.1002C11.1483 14.1002 14.1032 11.1453 14.1032 7.50023C14.1032 7.25044 14.0893 7.00389 14.0623 6.76131L14.0255 6.48407C13.991 6.26083 13.9453 6.04129 13.8891 5.82642C13.8213 5.56709 13.7382 5.31398 13.6409 5.06881L13.5279 4.80132L13.4507 4.63542L13.3766 4.48666C13.2178 4.17773 13.0353 3.88295 12.8312 3.60423L12.6782 3.40352L12.4793 3.16432L12.3157 2.98361L12.1961 2.85951L12.0355 2.70246L11.8134 2.50184L11.4925 2.24191L11.2483 2.06498L10.9562 1.87446L10.6346 1.68894L10.3073 1.52378L10.1938 1.47176L9.95488 1.3706L9.67791 1.2669L9.42566 1.1846L9.10075 1.09489L8.83599 1.03486L8.54406 0.98184ZM10.4032 5.30023C10.4032 4.27588 10.2002 3.29829 9.83244 2.40604C11.7623 3.28995 13.1032 5.23862 13.1032 7.50023C13.1032 10.593 10.596 13.1002 7.50322 13.1002C6.63646 13.1002 5.81597 12.9036 5.08355 12.5522C6.5419 12.0941 7.81081 11.2082 8.74322 10.0416C8.87963 10.2284 9.10028 10.3497 9.34928 10.3497C9.76349 10.3497 10.0993 10.0139 10.0993 9.59971C10.0993 9.24256 9.84965 8.94373 9.51535 8.86816C9.57741 8.75165 9.63653 8.63334 9.6926 8.51332C9.88358 8.63163 10.1088 8.69993 10.35 8.69993C11.0403 8.69993 11.6 8.14028 11.6 7.44993C11.6 6.75976 11.0406 6.20024 10.3505 6.19993C10.3853 5.90487 10.4032 5.60464 10.4032 5.30023Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>"##
}

/// Light theme logo path (dark text, for light backgrounds)
const LOGO_DARK_PATH: &str = "/public/logo-dark.png";

/// Dark theme logo path (light text, for dark backgrounds)
const LOGO_LIGHT_PATH: &str = "/public/logo-light.png";

/// Wallet icon SVG (lucide wallet, same as register.allfeat.org)
fn wallet_icon() -> &'static str {
    r##"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1"></path><path d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4"></path></svg>"##
}

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
    
    // Balance state
    let (balance, set_balance) = create_signal(None::<String>);
    // Theme state (true = dark, false = light)
    let (is_dark, set_is_dark) = create_signal(true);
    // Language dropdown state
    let (lang_dropdown_open, set_lang_dropdown_open) = create_signal(false);
    // Wallet modal state
    let (wallet_modal_open, set_wallet_modal_open) = create_signal(false);
    
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
                        src=move || if is_dark.get() { LOGO_LIGHT_PATH } else { LOGO_DARK_PATH }
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
                        <span class="lang-icon" inner_html=globe_icon()></span>
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
                        class:icon-active=move || !is_dark.get()
                        inner_html=sun_icon()
                    ></span>
                    <div class="theme-switch" class:checked=move || is_dark.get()></div>
                    <span 
                        class="theme-icon" 
                        class:icon-active=move || is_dark.get()
                        inner_html=moon_icon()
                    ></span>
                </div>
                
                // Wallet connection button
                <div 
                    class="wallet-status" 
                    class:connected=move || wallet_connected.get()
                    on:click=on_wallet_click
                >
                    <span class="wallet-icon" inner_html=wallet_icon()></span>
                    <span>
                        {move || if let Some(addr) = wallet_address.get() {
                            format!("{}...{}", &addr[0..6.min(addr.len())], &addr[addr.len().saturating_sub(4)..])
                        } else {
                            Translations::connect_wallet(lang.get()).to_string()
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
