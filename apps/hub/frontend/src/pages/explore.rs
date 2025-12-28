//! Explore page - View MIDDS on-chain
//! Browse registered musical works, recordings, and releases on the blockchain

use leptos::*;

use crate::i18n::t;

/// Explore page - Browse on-chain MIDDS
#[component]
pub fn ExplorePage() -> impl IntoView {
    // Search query
    let (search_query, set_search_query) = create_signal(String::new());
    // Selected filter
    let (filter, set_filter) = create_signal("all".to_string());
    
    view! {
        <div class="explore-page">
            <div class="page-header">
                <h1>{t("explore.title")}</h1>
                <p class="page-subtitle">{t("explore.subtitle")}</p>
            </div>
            
            // Search and filters
            <div class="explore-controls">
                <div class="search-box">
                    <span class="search-icon">"🔍"</span>
                    <input 
                        type="text"
                        placeholder=move || t("explore.search_placeholder")
                        class="search-input"
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        prop:value=search_query
                    />
                </div>
                
                <div class="filter-tabs">
                    <button 
                        class=move || if filter.get() == "all" { "filter-tab active" } else { "filter-tab" }
                        on:click=move |_| set_filter.set("all".to_string())
                    >
                        {t("explore.filter_all")}
                    </button>
                    <button 
                        class=move || if filter.get() == "works" { "filter-tab active" } else { "filter-tab" }
                        on:click=move |_| set_filter.set("works".to_string())
                    >
                        {t("explore.filter_works")}
                    </button>
                    <button 
                        class=move || if filter.get() == "recordings" { "filter-tab active" } else { "filter-tab" }
                        on:click=move |_| set_filter.set("recordings".to_string())
                    >
                        {t("explore.filter_recordings")}
                    </button>
                    <button 
                        class=move || if filter.get() == "releases" { "filter-tab active" } else { "filter-tab" }
                        on:click=move |_| set_filter.set("releases".to_string())
                    >
                        {t("explore.filter_releases")}
                    </button>
                </div>
            </div>
            
            // Stats cards
            <div class="explore-stats">
                <div class="stat-card">
                    <div class="stat-value">"--"</div>
                    <div class="stat-label">{t("explore.total_works")}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">"--"</div>
                    <div class="stat-label">{t("explore.total_recordings")}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">"--"</div>
                    <div class="stat-label">{t("explore.total_releases")}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">"--"</div>
                    <div class="stat-label">{t("explore.total_artists")}</div>
                </div>
            </div>
            
            // Results area (placeholder)
            <div class="explore-results">
                <div class="results-placeholder">
                    <span class="placeholder-icon">"🎵"</span>
                    <p>{t("explore.no_results")}</p>
                    <p class="hint">{t("explore.search_hint")}</p>
                </div>
            </div>
        </div>
    }
}

