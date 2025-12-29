//! Home page - Welcome to Allfeat Apps Hub

use leptos::*;
use leptos_router::*;

use crate::i18n::t;
use crate::components::icons::*;

/// Home page with feature cards
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            // Hero section
            <section class="home-hero">
                <h1 class="home-title">{move || t("home.welcome")}</h1>
                <p class="home-subtitle">{move || t("home.subtitle")}</p>
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
                            <span class="feature-badge soon">{move || t("common.coming_soon")}</span>
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
            
            // Stats section
            <section class="stats-section">
                <div class="stat-card">
                    <div class="stat-value">"--"</div>
                    <div class="stat-label">{move || t("home.stats.works")}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">"--"</div>
                    <div class="stat-label">{move || t("home.stats.creators")}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">{move || t("network.melodie")}</div>
                    <div class="stat-label">{move || t("home.stats.network")}</div>
                </div>
            </section>
        </div>
    }
}


