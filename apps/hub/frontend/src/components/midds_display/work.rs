//! Musical Work MIDDS display component

use leptos::*;
use serde_json::Value;
use crate::services::explorer::{MusicalWorkData, CreatorData};
use super::common::{format_party_id, MiddsHeader, MiddsField};

/// Convert JSON Value to MusicalWorkData for massload compatibility
fn value_to_musical_work_data(work: &Value) -> Option<MusicalWorkData> {
    log::debug!("🔍 Converting work JSON: {}", serde_json::to_string_pretty(work).unwrap_or_default());
    
    // Title is required - if missing, return None
    let title = work.get("title")
        .and_then(|v| v.as_str());
    
    if title.is_none() {
        log::error!("❌ Missing title field in work JSON");
        return None;
    }
    
    let result = MusicalWorkData {
        id: work.get("id")
            .and_then(|v| v.as_str().or_else(|| v.as_u64().map(|_| "generated")))
            .unwrap_or("0")
            .to_string(),
        title: title.unwrap().to_string(),
        iswc: work.get("iswc").and_then(|v| v.as_str()).map(String::from),
        creators: work.get("creators")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|c| {
                        let id = c.get("id")?;
                        
                        // Handle both "role" (massload) and "roles" (explore) formats
                        let roles = if let Some(role_str) = c.get("role").and_then(|r| r.as_str()) {
                            vec![role_str.to_string()]
                        } else if let Some(roles_arr) = c.get("roles").and_then(|r| r.as_array()) {
                            roles_arr.iter()
                                .filter_map(|r| r.as_str().map(String::from))
                                .collect()
                        } else {
                            vec![]
                        };
                        
                        Some(CreatorData {
                            name: String::new(), // Not displayed anymore
                            roles,
                            id: id.clone(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
        creation_year: work.get("creationYear").and_then(|v| v.as_u64()).map(|y| y as u16),
        // Handle both "instrumental" (massload) and "isInstrumental" (explore)
        is_instrumental: work.get("instrumental")
            .or_else(|| work.get("isInstrumental"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        work_type: work.get("workType")
            .and_then(|w| {
                // Handle both { "type": "Original" } and "Original" formats
                w.get("type").and_then(|t| t.as_str())
                    .or_else(|| w.as_str())
            })
            .unwrap_or("Unknown")
            .to_string(),
        language: work.get("language").and_then(|v| v.as_str()).map(String::from),
        // Handle both "key" (massload) and "musicalKey" (explore)
        musical_key: work.get("key")
            .or_else(|| work.get("musicalKey"))
            .and_then(|v| v.as_str())
            .map(String::from),
    };
    
    log::debug!("✅ Successfully converted work: {}", result.title);
    Some(result)
}

/// Display a single creator with roles
#[component]
fn CreatorItem(
    #[prop(into)] creator: CreatorData,
) -> impl IntoView {
    let id_display = format_party_id(&creator.id);
    
    view! {
        <div class="midds-array-item">
            <div class="midds-object">
                <div class="midds-prop">
                    <span class="prop-key">"id"</span>
                    <span class="prop-value ipi-code">{id_display}</span>
                </div>
                <div class="midds-prop">
                    <span class="prop-key">"role"</span>
                    <span class="prop-value">
                        {creator.roles.into_iter().map(|role| {
                            view! { <span class="role-badge">{role}</span> }
                        }).collect::<Vec<_>>()}
                    </span>
                </div>
            </div>
        </div>
    }
}

/// Display musical work from JSON Value (for massload compatibility)
#[component]
pub fn WorkDisplayJson(
    work: Value,
) -> impl IntoView {
    match value_to_musical_work_data(&work) {
        Some(work_data) => view! { <WorkDisplay work=work_data/> }.into_view(),
        None => view! { <div class="error">"Invalid work data"</div> }.into_view(),
    }
}

/// Display musical work in MIDDS format
#[component]
pub fn WorkDisplay(
    #[prop(into)] work: MusicalWorkData,
) -> impl IntoView {
    // Separate creators from publishers
    let (creators, publishers): (Vec<_>, Vec<_>) = work.creators.clone().into_iter()
        .partition(|c| !c.roles.iter().any(|r| r == "Publisher"));
    
    let creators_count = creators.len();
    let publishers_count = publishers.len();
    
    view! {
        <div class="midds-work">
            <MiddsHeader midds_type="MusicalWork"/>
            
            // Title
            <MiddsField label="title">
                {work.title}
            </MiddsField>
            
            // ISWC (optional)
            {work.iswc.map(|iswc| view! {
                <MiddsField label="iswc" class="iswc-value">
                    {iswc}
                </MiddsField>
            })}
            
            // Creators (non-publishers)
            <div class="midds-field">
                <div class="midds-label">
                    "creators" 
                    <span class="midds-count">"(" {creators_count} ")"</span>
                </div>
                <div class="midds-array">
                    <For
                        each=move || creators.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(_index, creator)| {
                            view! { <CreatorItem creator=creator/> }
                        }
                    />
                </div>
            </div>
            
            // Publishers (separate section)
            {if publishers_count > 0 {
                view! {
                    <div class="midds-field">
                        <div class="midds-label">
                            "publishers" 
                            <span class="midds-count">"(" {publishers_count} ")"</span>
                        </div>
                        <div class="midds-array">
                            <For
                                each=move || publishers.clone().into_iter().enumerate()
                                key=|(i, _)| *i
                                children=move |(_index, publisher)| {
                                    view! { <CreatorItem creator=publisher/> }
                                }
                            />
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
            
            // Creation Year (optional)
            {work.creation_year.map(|year| view! {
                <MiddsField label="creationYear">
                    {year}
                </MiddsField>
            })}
            
            // Work Type
            <MiddsField label="workType" class="type-value">
                {work.work_type}
            </MiddsField>
            
            // Instrumental
            <MiddsField label="instrumental" class="bool-value">
                {if work.is_instrumental { "true" } else { "false" }}
            </MiddsField>
            
            // Language (optional)
            {work.language.map(|lang| view! {
                <MiddsField label="language">
                    {lang.clone()}
                    <span class="midds-lang">{lang}</span>
                </MiddsField>
            })}
            
            // Musical Key (optional)
            {work.musical_key.map(|key| view! {
                <MiddsField label="key">
                    {key}
                </MiddsField>
            })}
        </div>
    }
}

