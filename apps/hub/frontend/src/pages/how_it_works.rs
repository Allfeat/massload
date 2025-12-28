//! How it works page - Explains Allfeat ecosystem

use leptos::*;
use leptos_router::*;

use crate::i18n::t;
use crate::components::icons::*;

/// How it works page
#[component]
pub fn HowItWorksPage() -> impl IntoView {
    view! {
        <div class="how-it-works-page">
            <div class="page-header">
                <h1>{t("how.title")}</h1>
                <p class="page-subtitle">{t("how.subtitle")}</p>
            </div>
            
            // Steps
            <div class="steps-container">
                // Step 1 - Register
                <div class="step-card">
                    <div class="step-number">"1"</div>
                    <div class="step-icon"><IconPenLine/></div>
                    <h3>{t("how.step1_title")}</h3>
                    <p>{t("how.step1_desc")}</p>
                </div>
                
                // Step 2 - Validate
                <div class="step-card">
                    <div class="step-number">"2"</div>
                    <div class="step-icon"><IconCheckCircle/></div>
                    <h3>{t("how.step2_title")}</h3>
                    <p>{t("how.step2_desc")}</p>
                </div>
                
                // Step 3 - Certify
                <div class="step-card">
                    <div class="step-number">"3"</div>
                    <div class="step-icon"><IconLink/></div>
                    <h3>{t("how.step3_title")}</h3>
                    <p>{t("how.step3_desc")}</p>
                </div>
                
                // Step 4 - Protect
                <div class="step-card">
                    <div class="step-number">"4"</div>
                    <div class="step-icon"><IconShield/></div>
                    <h3>{t("how.step4_title")}</h3>
                    <p>{t("how.step4_desc")}</p>
                </div>
            </div>
            
            // Key concepts
            <div class="concepts-section">
                <h2>{t("how.concepts_title")}</h2>
                
                <div class="concepts-grid">
                    <div class="concept-card">
                        <h4>"MIDDS"</h4>
                        <p>{t("how.midds_desc")}</p>
                    </div>
                    <div class="concept-card">
                        <h4>"ISWC"</h4>
                        <p>{t("how.iswc_desc")}</p>
                    </div>
                    <div class="concept-card">
                        <h4>"ISRC"</h4>
                        <p>{t("how.isrc_desc")}</p>
                    </div>
                    <div class="concept-card">
                        <h4>"IPI"</h4>
                        <p>{t("how.ipi_desc")}</p>
                    </div>
                </div>
            </div>
            
            // CTA
            <div class="cta-section">
                <h2>{t("how.cta_title")}</h2>
                <div class="cta-buttons">
                    <A href="/register" class="btn btn-primary">{t("how.start_registering")}</A>
                    <a href="https://docs.allfeat.org" target="_blank" class="btn btn-secondary">
                        {t("how.read_docs")}
                        <span class="btn-icon"><IconExternalLink/></span>
                    </a>
                </div>
            </div>
        </div>
    }
}

