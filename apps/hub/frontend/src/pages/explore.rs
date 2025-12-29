//! Explore page - View MIDDS on-chain
//! Browse registered musical works, recordings, and releases on the blockchain

use leptos::*;

use crate::i18n::t;
use crate::components::icons::*;
use crate::services::explorer::{fetch_blockchain_metrics, fetch_all_musical_works};
use crate::MusicalWorkData;

/// Explore page - Browse on-chain MIDDS
#[component]
pub fn ExplorePage() -> impl IntoView {
    // Search query
    let (search_query, set_search_query) = create_signal(String::new());
    // Selected filter
    let (filter, set_filter) = create_signal("all".to_string());
    
    // RPC URL from config
    let rpc_url_metrics = "wss://melodie-rpc.allfeat.io".to_string();
    let rpc_url_works = "wss://melodie-rpc.allfeat.io".to_string();
    
    // Fetch metrics
    let metrics = create_resource(
        || (),
        move |_| {
            let url = rpc_url_metrics.clone();
            async move {
                fetch_blockchain_metrics(&url).await
            }
        }
    );
    
    // Works state - loaded when "works" filter is selected
    let (works, set_works) = create_signal(None::<Vec<MusicalWorkData>>);
    let (is_loading_works, set_is_loading_works) = create_signal(false);
    
    // Effect to load works when filter changes
    create_effect(move |_| {
        let current_filter = filter.get();
        if current_filter == "works" && works.get().is_none() {
            set_is_loading_works.set(true);
            let url = rpc_url_works.clone();
            spawn_local(async move {
                match fetch_all_musical_works(&url).await {
                    Ok(works_vec) => {
                        log::info!("✅ Loaded {} works", works_vec.len());
                        set_works.set(Some(works_vec));
                    },
                    Err(e) => {
                        log::error!("❌ Failed to load works: {}", e);
                        set_works.set(Some(vec![]));
                    }
                }
                set_is_loading_works.set(false);
            });
        }
    });
    
    view! {
        <div class="explore-page">
            <div class="page-header">
                <h1>{t("explore.title")}</h1>
                <p class="page-subtitle">{t("explore.subtitle")}</p>
            </div>
            
            // Search and filters
            <div class="explore-controls">
                <div class="search-box">
                    <span class="search-icon"><IconSearch/></span>
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
                    <div class="stat-value">
                        {move || metrics.get().map(|m| m.as_ref().map(|m| m.works.to_string()).unwrap_or_else(|_| "--".to_string())).unwrap_or_else(|| "--".to_string())}
                    </div>
                    <div class="stat-label">{t("explore.total_works")}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">
                        {move || metrics.get().map(|m| m.as_ref().map(|m| m.recordings.to_string()).unwrap_or_else(|_| "--".to_string())).unwrap_or_else(|| "--".to_string())}
                    </div>
                    <div class="stat-label">{t("explore.total_recordings")}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">
                        {move || metrics.get().map(|m| m.as_ref().map(|m| m.releases.to_string()).unwrap_or_else(|_| "--".to_string())).unwrap_or_else(|| "--".to_string())}
                    </div>
                    <div class="stat-label">{t("explore.total_releases")}</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">"--"</div>
                    <div class="stat-label">{t("explore.total_artists")}</div>
                </div>
            </div>
            
            // Results area
            <div class="explore-results">
                {move || {
                    if filter.get() == "works" {
                        if is_loading_works.get() {
                            view! { 
                                <div class="results-placeholder"><p>"Chargement..."</p></div> 
                            }.into_view()
                        } else {
                            match works.get() {
                                Some(works_list) if !works_list.is_empty() => {
                                    let (expanded_index, set_expanded_index) = create_signal(None::<usize>);
                                    let works_count = works_list.len();
                                    
                                    view! {
                                                <div class="preview-section show">
                                                    <div class="preview-list">
                                                        <For
                                                            each=move || works.get().unwrap_or_default().into_iter().enumerate()
                                                            key=|(idx, _)| *idx
                                                            children=move |(idx, work)| {
                                                                let is_expanded = move || expanded_index.get() == Some(idx);
                                                                
                                                                let toggle_expand = move |_| {
                                                                    if expanded_index.get() == Some(idx) {
                                                                        set_expanded_index.set(None);
                                                                    } else {
                                                                        set_expanded_index.set(Some(idx));
                                                                    }
                                                                };
                                                                
                                                                view! {
                                                                    <div class="preview-item" class:expanded=is_expanded>
                                                                        <div class="preview-item-header" on:click=toggle_expand style="cursor: pointer;">
                                                                            <div class="preview-item-title">
                                                                                {if is_expanded() { "▼ " } else { "▶ " }}
                                                                                {work.title.clone()}
                                                                            </div>
                                                                            <div class="preview-item-details">
                                                                                "ISWC: " {work.iswc.clone().unwrap_or_else(|| "-".to_string())} 
                                                                                " • Créateurs: " {work.creators.len()}
                                                                            </div>
                                                                        </div>
                                                                        
                                                                        <Show
                                                                            when=is_expanded
                                                                            fallback=|| view! { }
                                                                        >
                                                                            <div class="preview-item-expanded">
                                                                                <div class="midds-work">
                                                                                    <div class="midds-header">
                                                                                        <span class="midds-badge">"MIDDS"</span>
                                                                                        <span class="midds-type">"MusicalWork"</span>
                                                                                    </div>
                                                                                    
                                                                                    <div class="midds-field">
                                                                                        <div class="midds-label">"title"</div>
                                                                                        <div class="midds-value">{work.title.clone()}</div>
                                                                                    </div>
                                                                                    
                                                                                    {work.iswc.clone().map(|iswc| view! {
                                                                                        <div class="midds-field">
                                                                                            <div class="midds-label">"iswc"</div>
                                                                                            <div class="midds-value iswc-value">{iswc}</div>
                                                                                        </div>
                                                                                    })}
                                                                                    
                                                                                    <div class="midds-field">
                                                                                        <div class="midds-label">"creators" <span class="midds-count">"(" {work.creators.len()} ")"</span></div>
                                                                                        <div class="midds-array">
                                                                                            {work.creators.iter().enumerate().map(|(i, creator)| {
                                                                                                view! {
                                                                                                    <div class="midds-array-item">
                                                                                                        <span class="midds-index">{i}</span>
                                                                                                        <div class="midds-object">
                                                                                                            <div class="midds-prop">
                                                                                                                <span class="prop-key">"name"</span>
                                                                                                                <span class="prop-value">{creator.name.clone()}</span>
                                                                                                            </div>
                                                                                                            <div class="midds-prop">
                                                                                                                <span class="prop-key">"roles"</span>
                                                                                                                <span class="prop-value">
                                                                                                                    {creator.roles.iter().map(|r| view! {
                                                                                                                        <span class="role-badge">{r.clone()}</span>
                                                                                                                    }).collect::<Vec<_>>()}
                                                                                                                </span>
                                                                                                            </div>
                                                                                                        </div>
                                                                                                    </div>
                                                                                                }
                                                                                            }).collect::<Vec<_>>()}
                                                                                        </div>
                                                                                    </div>
                                                                                    
                                                                                    {work.creation_year.map(|year| view! {
                                                                                        <div class="midds-field">
                                                                                            <div class="midds-label">"creation_year"</div>
                                                                                            <div class="midds-value">{year}</div>
                                                                                        </div>
                                                                                    })}
                                                                                    
                                                                                    <div class="midds-field">
                                                                                        <div class="midds-label">"work_type"</div>
                                                                                        <div class="midds-value type-value">{work.work_type.clone()}</div>
                                                                                    </div>
                                                                                </div>
                                                                            </div>
                                                                        </Show>
                                                                    </div>
                                                                }
                                                            }
                                                        />
                                                    </div>
                                                    
                                                    <div class="preview-footer">
                                                        <div class="preview-cost">
                                                            <strong>{works_count}</strong> " œuvres enregistrées"
                                                        </div>
                                                    </div>
                                                </div>
                                            }.into_view()
                                        }
                                        _ => {
                                            view! {
                                                <div class="results-placeholder">
                                                    <span class="placeholder-icon"><IconMusic/></span>
                                                    <p>{t("explore.no_results")}</p>
                                                    <p class="hint">{t("explore.search_hint")}</p>
                                                </div>
                                            }.into_view()
                                        }
                                    }
                        }
                    } else {
                        view! {
                            <div class="results-placeholder">
                                <span class="placeholder-icon"><IconMusic/></span>
                                <p>{t("explore.no_results")}</p>
                                <p class="hint">{t("explore.search_hint")}</p>
                            </div>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}

