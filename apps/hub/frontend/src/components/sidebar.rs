//! Sidebar navigation component - Tanssi-style left menu

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
            <div class="sidebar-header">
                <a href="/" class="sidebar-logo">
                    <img src="/logo-light.png" alt="Allfeat" class="logo-img"/>
                    <span class="logo-text">"Allfeat"</span>
                </a>
            </div>
            
            <nav class="sidebar-nav">
                // Main section
                <div class="nav-section">
                    <A 
                        href="/" 
                        class=move || if is_active("/") && !is_active("/massload") && !is_active("/register") && !is_active("/protect") { 
                            "nav-item active" 
                        } else { 
                            "nav-item" 
                        }
                    >
                        <span class="nav-icon">"🏠"</span>
                        <span class="nav-label">{t("nav.home")}</span>
                    </A>
                </div>
                
                // Apps section
                <div class="nav-section">
                    <div class="nav-section-title">{t("nav.apps")}</div>
                    
                    <A 
                        href="/massload" 
                        class=move || if is_active("/massload") { "nav-item active" } else { "nav-item" }
                    >
                        <span class="nav-icon">"📦"</span>
                        <span class="nav-label">{t("nav.massload")}</span>
                        <span class="nav-badge live">"Live"</span>
                    </A>
                    
                    <A 
                        href="/register" 
                        class=move || if is_active("/register") { "nav-item active" } else { "nav-item" }
                    >
                        <span class="nav-icon">"✍️"</span>
                        <span class="nav-label">{t("nav.register")}</span>
                        <span class="nav-badge soon">"Soon"</span>
                    </A>
                    
                    <A 
                        href="/protect" 
                        class=move || if is_active("/protect") { "nav-item active" } else { "nav-item" }
                    >
                        <span class="nav-icon">"🛡️"</span>
                        <span class="nav-label">{t("nav.protect")}</span>
                        <span class="nav-badge soon">"Soon"</span>
                    </A>
                </div>
                
                // Explore section
                <div class="nav-section">
                    <div class="nav-section-title">{t("nav.explore")}</div>
                    
                    <a href="https://docs.allfeat.org" target="_blank" class="nav-item external">
                        <span class="nav-icon">"📚"</span>
                        <span class="nav-label">{t("nav.docs")}</span>
                        <span class="nav-external">"↗"</span>
                    </a>
                    
                    <a href="https://allfeat.org" target="_blank" class="nav-item external">
                        <span class="nav-icon">"🌐"</span>
                        <span class="nav-label">{t("nav.website")}</span>
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

