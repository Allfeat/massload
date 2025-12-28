//! Sidebar navigation component - Tanssi-style left menu
//! Based on https://apps.tanssi.network/

use leptos::*;
use leptos_router::*;

use crate::i18n::t;

/// Sidebar navigation with Tanssi-style menu
#[component]
pub fn Sidebar() -> impl IntoView {
    let location = use_location();
    
    // Check if a path is active
    let is_active = move |path: &str| {
        let current = location.pathname.get();
        if path == "/" {
            current == "/"
        } else {
            current.starts_with(path)
        }
    };

    view! {
        <aside class="sidebar">
            <nav class="sidebar-nav">
                // Home
                <div class="nav-section">
                    <A 
                        href="/" 
                        class=move || if is_active("/") && !is_active("/explore") && !is_active("/register") && !is_active("/protect") && !is_active("/massload") && !is_active("/how-it-works") { 
                            "nav-item active" 
                        } else { 
                            "nav-item" 
                        }
                    >
                        <span class="nav-icon">"🏠"</span>
                        <span class="nav-label">{t("nav.home")}</span>
                    </A>
                </div>
                
                // Explore - View MIDDS on-chain
                <div class="nav-section">
                    <A 
                        href="/explore" 
                        class=move || if is_active("/explore") { "nav-item active" } else { "nav-item" }
                    >
                        <span class="nav-icon">"🔍"</span>
                        <span class="nav-label">{t("nav.explore")}</span>
                    </A>
                </div>
                
                // Register - like register.allfeat.org
                <div class="nav-section">
                    <A 
                        href="/register" 
                        class=move || if is_active("/register") { "nav-item active" } else { "nav-item" }
                    >
                        <span class="nav-icon">"✍️"</span>
                        <span class="nav-label">{t("nav.register")}</span>
                    </A>
                </div>
                
                // Protect - Timestamp/ATS
                <div class="nav-section">
                    <A 
                        href="/protect" 
                        class=move || if is_active("/protect") { "nav-item active" } else { "nav-item" }
                    >
                        <span class="nav-icon">"🛡️"</span>
                        <span class="nav-label">{t("nav.protect")}</span>
                    </A>
                </div>
                
                // Mass Load - Bulk import (hidden but accessible)
                <div class="nav-section">
                    <A 
                        href="/massload" 
                        class=move || if is_active("/massload") { "nav-item active" } else { "nav-item" }
                    >
                        <span class="nav-icon">"📦"</span>
                        <span class="nav-label">{t("nav.massload")}</span>
                    </A>
                </div>
                
                // Separator
                <div class="nav-divider"></div>
                
                // How it works
                <div class="nav-section">
                    <A 
                        href="/how-it-works" 
                        class=move || if is_active("/how-it-works") { "nav-item active" } else { "nav-item" }
                    >
                        <span class="nav-icon">"📖"</span>
                        <span class="nav-label">{t("nav.how_it_works")}</span>
                    </A>
                </div>
                
                // External links
                <div class="nav-section">
                    <a href="https://docs.allfeat.org" target="_blank" class="nav-item external">
                        <span class="nav-icon">"📚"</span>
                        <span class="nav-label">{t("nav.docs")}</span>
                        <span class="nav-external">"↗"</span>
                    </a>
                </div>
            </nav>
            
            <div class="sidebar-footer">
                <div class="network-status">
                    <span class="status-dot online"></span>
                    <span class="status-text">"Melodie Testnet"</span>
                </div>
            </div>
        </aside>
    }
}
