//! Register page - Single work registration (coming soon)

use leptos::*;

use crate::i18n::t;

/// Register page placeholder
#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <div class="coming-soon-page">
            <div class="coming-soon-content">
                <div class="coming-soon-icon">"✍️"</div>
                <h1 class="coming-soon-title">{t("register.title")}</h1>
                <p class="coming-soon-description">{t("register.coming_soon")}</p>
                
                <div class="coming-soon-features">
                    <h3>{t("register.features_title")}</h3>
                    <ul>
                        <li>{t("register.feature1")}</li>
                        <li>{t("register.feature2")}</li>
                        <li>{t("register.feature3")}</li>
                    </ul>
                </div>
                
                <a href="https://register.allfeat.org" target="_blank" class="coming-soon-link">
                    {t("register.current_version")}
                    <span class="link-arrow">"↗"</span>
                </a>
            </div>
        </div>
    }
}

