//! Protect page - IP protection features (coming soon)

use leptos::*;

use crate::i18n::t;

/// Protect page placeholder
#[component]
pub fn ProtectPage() -> impl IntoView {
    view! {
        <div class="coming-soon-page">
            <div class="coming-soon-content">
                <div class="coming-soon-icon">"🛡️"</div>
                <h1 class="coming-soon-title">{t("protect.title")}</h1>
                <p class="coming-soon-description">{t("protect.coming_soon")}</p>
                
                <div class="coming-soon-features">
                    <h3>{t("protect.features_title")}</h3>
                    <ul>
                        <li>{t("protect.feature1")}</li>
                        <li>{t("protect.feature2")}</li>
                        <li>{t("protect.feature3")}</li>
                    </ul>
                </div>
                
                <a href="https://protect.allfeat.org" target="_blank" class="coming-soon-link">
                    {t("protect.current_version")}
                    <span class="link-arrow">"↗"</span>
                </a>
            </div>
        </div>
    }
}

