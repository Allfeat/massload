//! Register page - Register MIDDS on-chain
//! Based on https://register.allfeat.org/fr

use leptos::*;
use leptos_router::*;

use crate::i18n::t;
use crate::components::icons::*;

/// Register hub page with cards grouped like register.allfeat.org
#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <div class="register-page">
            // Hero section
            <div class="register-hero">
                <h1>{t("register.hero_title")}</h1>
                <p class="register-subtitle">{t("register.hero_subtitle")}</p>
            </div>
            
            // Card groups container
            <div class="register-groups">
                // Group 1: Party Identification
                <div class="register-group">
                    <div class="group-header">
                        <span class="group-icon"><IconUsers/></span>
                        <span class="group-title">{t("register.group_parties")}</span>
                    </div>
                    <div class="group-cards">
                        <A href="/register-artist" class="midds-card coming-soon">
                            <span class="card-icon"><IconUser/></span>
                            <span class="card-label">{t("register.artist.title")}</span>
                        </A>
                        <A href="/register-legal-entity" class="midds-card coming-soon">
                            <span class="card-icon"><IconBuilding/></span>
                            <span class="card-label">{t("register.legal_entity.title")}</span>
                        </A>
                    </div>
                    <p class="group-note">{t("register.parties_note")}</p>
                </div>
                
                // Group 2: Product Identification
                <div class="register-group">
                    <div class="group-header">
                        <span class="group-icon"><IconMusic/></span>
                        <span class="group-title">{t("register.group_products")}</span>
                    </div>
                    <div class="group-cards">
                        <A href="/register-musical-work" class="midds-card">
                            <span class="card-icon"><IconMusic/></span>
                            <span class="card-label">{t("register.musical_work.title")}</span>
                        </A>
                        <A href="/register-recording" class="midds-card">
                            <span class="card-icon"><IconMic/></span>
                            <span class="card-label">{t("register.recording.title")}</span>
                        </A>
                        <A href="/register-release" class="midds-card">
                            <span class="card-icon"><IconDisc/></span>
                            <span class="card-label">{t("register.release.title")}</span>
                        </A>
                    </div>
                </div>
            </div>
            
            // Mass Load section (below register cards)
            <div class="massload-section">
                <div class="massload-card">
                    <div class="massload-content">
                        <span class="massload-icon"><IconPackage/></span>
                        <div class="massload-text">
                            <h3>{t("register.massload_title")}</h3>
                            <p>{t("register.massload_desc")}</p>
                        </div>
                    </div>
                    <A href="/massload" class="massload-btn">
                        {t("register.go_to_massload")}
                        <IconArrowRight/>
                    </A>
                </div>
            </div>
        </div>
    }
}

/// Musical Work registration form
#[component]
pub fn RegisterMusicalWorkPage() -> impl IntoView {
    use crate::components::musical_work_form_v2::MusicalWorkFormV2;
    
    view! {
        <MusicalWorkFormV2 />
    }
}


/// Recording registration form  
#[component]
pub fn RegisterRecordingPage() -> impl IntoView {
    view! {
        <div class="register-form-page">
            <div class="form-header">
                <A href="/register" class="back-link">"← " {t("register.back")}</A>
                <h1>{t("register.recording.title")}</h1>
                <p>{t("register.recording.form_desc")}</p>
            </div>
            
            <div class="form-container">
                <div class="form-section">
                    <h3>{t("register.form.basic_info")}</h3>
                    
                    <div class="form-group">
                        <label for="isrc">{t("register.form.isrc")} " *"</label>
                        <input type="text" id="isrc" class="form-input" placeholder="USRC10000001" />
                    </div>
                    
                    <div class="form-group">
                        <label for="work_iswc">{t("register.form.linked_work")}</label>
                        <input type="text" id="work_iswc" class="form-input" placeholder="T-000.000.000-0" />
                        <span class="form-hint">{t("register.form.linked_work_hint")}</span>
                    </div>
                    
                    <div class="form-group">
                        <label for="duration">{t("register.form.duration")}</label>
                        <input type="text" id="duration" class="form-input" placeholder="3:45" />
                    </div>
                </div>
                
                <div class="form-actions">
                    <button class="btn btn-primary" disabled>
                        {t("register.form.submit")}
                    </button>
                    <span class="form-notice">{t("register.form.wallet_required")}</span>
                </div>
            </div>
        </div>
    }
}

/// Release registration form
#[component]
pub fn RegisterReleasePage() -> impl IntoView {
    view! {
        <div class="register-form-page">
            <div class="form-header">
                <A href="/register" class="back-link">"← " {t("register.back")}</A>
                <h1>{t("register.release.title")}</h1>
                <p>{t("register.release.form_desc")}</p>
            </div>
            
            <div class="form-container">
                <div class="form-section">
                    <h3>{t("register.form.basic_info")}</h3>
                    
                    <div class="form-group">
                        <label for="title">{t("register.form.release_title")} " *"</label>
                        <input type="text" id="title" class="form-input" />
                    </div>
                    
                    <div class="form-row">
                        <div class="form-group">
                            <label for="release_date">{t("register.form.release_date")}</label>
                            <input type="date" id="release_date" class="form-input" />
                        </div>
                        <div class="form-group">
                            <label for="upc">{t("register.form.upc")}</label>
                            <input type="text" id="upc" class="form-input" placeholder="0123456789012" />
                        </div>
                    </div>
                </div>
                
                <div class="form-actions">
                    <button class="btn btn-primary" disabled>
                        {t("register.form.submit")}
                    </button>
                    <span class="form-notice">{t("register.form.wallet_required")}</span>
                </div>
            </div>
        </div>
    }
}

/// Artist registration (coming soon)
#[component]
pub fn RegisterArtistPage() -> impl IntoView {
    view! {
        <div class="coming-soon-page">
            <div class="coming-soon-content">
                <div class="coming-soon-icon">"👤"</div>
                <h1 class="coming-soon-title">{t("register.artist.title")}</h1>
                <p class="coming-soon-description">{t("register.artist.coming_soon")}</p>
                <A href="/register" class="coming-soon-link">
                    "← " {t("register.back")}
                </A>
            </div>
        </div>
    }
}

/// Legal Entity registration (coming soon)
#[component]
pub fn RegisterLegalEntityPage() -> impl IntoView {
    view! {
        <div class="coming-soon-page">
            <div class="coming-soon-content">
                <div class="coming-soon-icon">"🏢"</div>
                <h1 class="coming-soon-title">{t("register.legal_entity.title")}</h1>
                <p class="coming-soon-description">{t("register.legal_entity.coming_soon")}</p>
                <A href="/register" class="coming-soon-link">
                    "← " {t("register.back")}
                </A>
            </div>
        </div>
    }
}
