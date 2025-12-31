//! Home page - Welcome to Allfeat Apps Hub

use leptos::*;
use leptos_router::*;

use crate::i18n::t;
use crate::components::icons::*;
use crate::network::{use_rpc_url, use_network};
use crate::services::explorer::fetch_blockchain_metrics;

/// Home page with feature cards
#[component]
pub fn HomePage() -> impl IntoView {
    // Get current network and RPC URL
    let network = use_network();
    let rpc_url = use_rpc_url();
    
    // Fetch blockchain metrics
    let metrics = create_resource(
        move || rpc_url.get(),
        move |url| async move {
            fetch_blockchain_metrics(&url).await.ok()
        }
    );
    
    // Total works is already calculated (works + recordings + releases)
    let total_works = move || {
        metrics.get()
            .and_then(|m| m.map(|metrics| metrics.total))
    };
    view! {
        <div class="home-page">
            // Hero section
            <section class="home-hero">
                <h1 class="home-title">{move || t("home.welcome")}</h1>
                <p class="home-subtitle">{move || t("home.subtitle")}</p>
                
                // Hero stats - Social proof (live data from blockchain)
                <div class="hero-stats">
                    <div class="hero-stat">
                        <div class="hero-stat-value">
                            {move || total_works()
                                .map(|n| n.to_string())
                                .unwrap_or_else(|| "--".to_string())
                            }
                        </div>
                        <div class="hero-stat-label">{move || t("home.stats.works")}</div>
                    </div>
                    <div class="hero-stat">
                        <div class="hero-stat-value">"--"</div>
                        <div class="hero-stat-label">{move || t("home.stats.creators")}</div>
                    </div>
                    <div class="hero-stat">
                        <div class="hero-stat-value">{move || network.get().name()}</div>
                        <div class="hero-stat-label">{move || t("home.stats.network")}</div>
                    </div>
                </div>
            </section>
            
            // Feature cards
            <section class="feature-cards">
                <A href="/massload" class="feature-card">
                    <div class="feature-icon"><IconPackage/></div>
                    <div class="feature-content">
                        <div class="feature-header">
                            <h3 class="feature-title">{move || t("home.massload.title")}</h3>
                            <span class="feature-badge live">{move || t("common.live")}</span>
                        </div>
                        <p class="feature-description">{move || t("home.massload.desc")}</p>
                    </div>
                    <div class="feature-arrow"><IconArrowRight/></div>
                </A>
                
                <A href="/register" class="feature-card">
                    <div class="feature-icon"><IconPenLine/></div>
                    <div class="feature-content">
                        <div class="feature-header">
                            <h3 class="feature-title">{move || t("home.register.title")}</h3>
                            <span class="feature-badge live">{move || t("common.live")}</span>
                        </div>
                        <p class="feature-description">{move || t("home.register.desc")}</p>
                    </div>
                    <div class="feature-arrow"><IconArrowRight/></div>
                </A>
                
                <A href="/protect" class="feature-card">
                    <div class="feature-icon"><IconShield/></div>
                    <div class="feature-content">
                        <div class="feature-header">
                            <h3 class="feature-title">{move || t("home.protect.title")}</h3>
                            <span class="feature-badge live">{move || t("common.live")}</span>
                        </div>
                        <p class="feature-description">{move || t("home.protect.desc")}</p>
                    </div>
                    <div class="feature-arrow"><IconArrowRight/></div>
                </A>
            </section>
            
            // Resources section
            <section class="resources-section">
                <h2 class="section-title">{move || t("home.resources.title")}</h2>
                
                <div class="resources-grid">
                    <a href="https://docs.allfeat.org/getting-started" target="_blank" class="resource-link">
                        <span class="resource-icon"><IconRocket/></span>
                        <span class="resource-title">{move || t("home.resources.getting_started")}</span>
                        <span class="resource-arrow"><IconExternalLink/></span>
                    </a>
                    <a href="https://docs.allfeat.org" target="_blank" class="resource-link">
                        <span class="resource-icon"><IconBookOpen/></span>
                        <span class="resource-title">{move || t("home.resources.documentation")}</span>
                        <span class="resource-arrow"><IconExternalLink/></span>
                    </a>
                    <a href="https://t.me/Allfeat_fndn" target="_blank" class="resource-link">
                        <span class="resource-icon"><IconMessageCircle/></span>
                        <span class="resource-title">{move || t("home.resources.community")}</span>
                        <span class="resource-arrow"><IconExternalLink/></span>
                    </a>
                    <a href="https://github.com/allfeat" target="_blank" class="resource-link">
                        <span class="resource-icon"><IconGithub/></span>
                        <span class="resource-title">{move || t("home.resources.github")}</span>
                        <span class="resource-arrow"><IconExternalLink/></span>
                    </a>
                </div>
            </section>
        </div>
    }
}


