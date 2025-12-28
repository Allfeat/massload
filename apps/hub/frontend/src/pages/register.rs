//! Register page - Register MIDDS on-chain
//! Based on https://register.allfeat.org/fr

use leptos::*;
use leptos_router::*;

use crate::i18n::t;

/// Register hub page with cards for each MIDDS type
#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <div class="register-page">
            <div class="page-header">
                <h1>{t("register.title")}</h1>
                <p class="page-subtitle">{t("register.subtitle")}</p>
            </div>
            
            // Registration type cards
            <div class="register-cards">
                // Musical Work
                <A href="/register/musical-work" class="register-card">
                    <div class="card-icon">"🎵"</div>
                    <div class="card-content">
                        <h3>{t("register.musical_work.title")}</h3>
                        <p>{t("register.musical_work.desc")}</p>
                    </div>
                    <div class="card-arrow">"→"</div>
                </A>
                
                // Recording
                <A href="/register/recording" class="register-card">
                    <div class="card-icon">"🎙️"</div>
                    <div class="card-content">
                        <h3>{t("register.recording.title")}</h3>
                        <p>{t("register.recording.desc")}</p>
                    </div>
                    <div class="card-arrow">"→"</div>
                </A>
                
                // Release
                <A href="/register/release" class="register-card">
                    <div class="card-icon">"💿"</div>
                    <div class="card-content">
                        <h3>{t("register.release.title")}</h3>
                        <p>{t("register.release.desc")}</p>
                    </div>
                    <div class="card-arrow">"→"</div>
                </A>
                
                // Artist
                <A href="/register/artist" class="register-card coming-soon">
                    <div class="card-icon">"👤"</div>
                    <div class="card-content">
                        <h3>{t("register.artist.title")} <span class="badge soon">"Soon"</span></h3>
                        <p>{t("register.artist.desc")}</p>
                    </div>
                    <div class="card-arrow">"→"</div>
                </A>
                
                // Legal Entity
                <A href="/register/legal-entity" class="register-card coming-soon">
                    <div class="card-icon">"🏢"</div>
                    <div class="card-content">
                        <h3>{t("register.legal_entity.title")} <span class="badge soon">"Soon"</span></h3>
                        <p>{t("register.legal_entity.desc")}</p>
                    </div>
                    <div class="card-arrow">"→"</div>
                </A>
            </div>
            
            // Mass registration CTA
            <div class="mass-registration-cta">
                <div class="cta-content">
                    <span class="cta-icon">"📦"</span>
                    <div>
                        <h3>{t("register.mass_cta_title")}</h3>
                        <p>{t("register.mass_cta_desc")}</p>
                    </div>
                </div>
                <A href="/massload" class="btn btn-secondary">
                    {t("register.go_to_massload")}
                    <span>" →"</span>
                </A>
            </div>
        </div>
    }
}

/// Musical Work registration form
#[component]
pub fn RegisterMusicalWorkPage() -> impl IntoView {
    view! {
        <div class="register-form-page">
            <div class="form-header">
                <A href="/register" class="back-link">"← " {t("register.back")}</A>
                <h1>{t("register.musical_work.title")}</h1>
                <p>{t("register.musical_work.form_desc")}</p>
            </div>
            
            <div class="form-container">
                <div class="form-section">
                    <h3>{t("register.form.basic_info")}</h3>
                    
                    <div class="form-group">
                        <label for="title">{t("register.form.title")} " *"</label>
                        <input type="text" id="title" class="form-input" placeholder=t("register.form.title_placeholder") />
                    </div>
                    
                    <div class="form-group">
                        <label for="iswc">{t("register.form.iswc")}</label>
                        <input type="text" id="iswc" class="form-input" placeholder="T-000.000.000-0" />
                        <span class="form-hint">{t("register.form.iswc_hint")}</span>
                    </div>
                </div>
                
                <div class="form-section">
                    <h3>{t("register.form.creators")}</h3>
                    <p class="section-desc">{t("register.form.creators_desc")}</p>
                    
                    <div class="creator-list">
                        <div class="creator-item">
                            <input type="text" class="form-input" placeholder=t("register.form.creator_name") />
                            <select class="form-select">
                                <option value="composer">{t("register.form.role_composer")}</option>
                                <option value="author">{t("register.form.role_author")}</option>
                                <option value="arranger">{t("register.form.role_arranger")}</option>
                            </select>
                            <input type="text" class="form-input" placeholder="IPI (optional)" />
                        </div>
                    </div>
                    
                    <button class="btn btn-outline add-creator">
                        "+" {t("register.form.add_creator")}
                    </button>
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
