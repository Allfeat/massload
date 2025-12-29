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
            
            // Protocol introduction
            <div class="protocol-intro">
                <p class="intro-text">{t("how.intro_text")}</p>
                <a href="https://docs.allfeat.org/learn/metadata/" target="_blank" class="btn btn-link">
                    {t("how.learn_more")}
                    <span class="btn-icon"><IconExternalLink/></span>
                </a>
            </div>
            
            // Core principles
            <div class="principles-section">
                <h2>{t("how.principles_title")}</h2>
                <div class="principles-grid">
                    <div class="principle-card">
                        <div class="principle-icon"><IconUsers/></div>
                        <h4>{t("how.principle1_title")}</h4>
                        <p>{t("how.principle1_desc")}</p>
                    </div>
                    <div class="principle-card">
                        <div class="principle-icon"><IconCheckCircle/></div>
                        <h4>{t("how.principle2_title")}</h4>
                        <p>{t("how.principle2_desc")}</p>
                    </div>
                    <div class="principle-card">
                        <div class="principle-icon"><IconShield/></div>
                        <h4>{t("how.principle3_title")}</h4>
                        <p>{t("how.principle3_desc")}</p>
                    </div>
                </div>
            </div>
            
            // Certification Process with Proof of Metadata
            <div class="certification-section">
                <h2>{t("how.cert_title")}</h2>
                <p class="section-intro">{t("how.cert_intro")}</p>
                
                // Process steps
                <div class="process-steps">
                    <div class="process-step">
                        <div class="process-step-number">"1"</div>
                        <div class="step-icon"><IconPenLine/></div>
                        <h4>{t("how.cert_step1_title")}</h4>
                        <p>{t("how.cert_step1_desc")}</p>
                    </div>
                    
                    <div class="process-step">
                        <div class="process-step-number">"2"</div>
                        <div class="step-icon"><IconUsers/></div>
                        <h4>{t("how.cert_step2_title")}</h4>
                        <p>{t("how.cert_step2_desc")}</p>
                    </div>
                    
                    <div class="process-step">
                        <div class="process-step-number">"3"</div>
                        <div class="step-icon"><IconCheckCircle/></div>
                        <h4>{t("how.cert_step3_title")}</h4>
                        <p>{t("how.cert_step3_desc")}</p>
                    </div>
                </div>
                
                // Roles
                <div class="roles-section">
                    <h3>{t("how.roles_title")}</h3>
                    <div class="roles-grid">
                        <div class="role-card">
                            <h4>{t("how.role_provider_title")}</h4>
                            <p>{t("how.role_provider_desc")}</p>
                        </div>
                        <div class="role-card">
                            <h4>{t("how.role_truster_title")}</h4>
                            <p>{t("how.role_truster_desc")}</p>
                        </div>
                    </div>
                </div>
                
                // Advantages
                <div class="advantages-section">
                    <h3>{t("how.advantages_title")}</h3>
                    <ul class="advantages-list">
                        <li><strong>{t("how.adv1_title")}</strong> {t("how.adv1_desc")}</li>
                        <li><strong>{t("how.adv2_title")}</strong> {t("how.adv2_desc")}</li>
                        <li><strong>{t("how.adv3_title")}</strong> {t("how.adv3_desc")}</li>
                    </ul>
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

