//! Home page - Welcome to Allfeat Apps Hub

use leptos::*;
use leptos_router::*;

use crate::i18n::t;

/// Home page with feature cards
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            // Hero section
            <section class="home-hero">
                <h1 class="home-title">{t("home.welcome")}</h1>
                <p class="home-subtitle">{t("home.subtitle")}</p>
            </section>
            
            // Feature cards
            <section class="feature-cards">
                <FeatureCard 
                    href="/massload"
                    icon="📦"
                    title=t("home.massload.title")
                    description=t("home.massload.desc")
                    badge="Live"
                    badge_class="live"
                />
                
                <FeatureCard 
                    href="/register"
                    icon="✍️"
                    title=t("home.register.title")
                    description=t("home.register.desc")
                    badge="Coming Soon"
                    badge_class="soon"
                />
                
                <FeatureCard 
                    href="/protect"
                    icon="🛡️"
                    title=t("home.protect.title")
                    description=t("home.protect.desc")
                    badge="Coming Soon"
                    badge_class="soon"
                />
            </section>
            
            // Resources section
            <section class="resources-section">
                <h2 class="section-title">{t("home.resources.title")}</h2>
                
                <div class="resources-grid">
                    <ResourceLink 
                        href="https://docs.allfeat.org/getting-started"
                        icon="🚀"
                        title=t("home.resources.getting_started")
                    />
                    <ResourceLink 
                        href="https://docs.allfeat.org"
                        icon="📚"
                        title=t("home.resources.documentation")
                    />
                    <ResourceLink 
                        href="https://t.me/Allfeat_fndn"
                        icon="💬"
                        title=t("home.resources.community")
                    />
                    <ResourceLink 
                        href="https://github.com/allfeat"
                        icon="🔧"
                        title=t("home.resources.github")
                    />
                </div>
            </section>
            
            // Stats section
            <section class="stats-section">
                <div class="stat-card">
                    <div class="stat-value">"--"</div>
                    <div class="stat-label">{t("home.stats.works")}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">"--"</div>
                    <div class="stat-label">{t("home.stats.creators")}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">"Melodie"</div>
                    <div class="stat-label">{t("home.stats.network")}</div>
                </div>
            </section>
        </div>
    }
}

/// Feature card component
#[component]
fn FeatureCard(
    href: &'static str,
    icon: &'static str,
    title: String,
    description: String,
    badge: &'static str,
    badge_class: &'static str,
) -> impl IntoView {
    view! {
        <A href=href class="feature-card">
            <div class="feature-icon">{icon}</div>
            <div class="feature-content">
                <div class="feature-header">
                    <h3 class="feature-title">{title}</h3>
                    <span class=format!("feature-badge {}", badge_class)>{badge}</span>
                </div>
                <p class="feature-description">{description}</p>
            </div>
            <div class="feature-arrow">"→"</div>
        </A>
    }
}

/// Resource link component
#[component]
fn ResourceLink(
    href: &'static str,
    icon: &'static str,
    title: String,
) -> impl IntoView {
    view! {
        <a href=href target="_blank" class="resource-link">
            <span class="resource-icon">{icon}</span>
            <span class="resource-title">{title}</span>
            <span class="resource-arrow">"↗"</span>
        </a>
    }
}

