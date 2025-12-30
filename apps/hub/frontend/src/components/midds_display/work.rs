//! Musical Work MIDDS display component

use leptos::*;
use serde_json::Value;
use crate::services::explorer::{MusicalWorkData, CreatorData};
use super::common::{format_party_id, MiddsHeader, MiddsField};

/// Convert JSON Value to MusicalWorkData for massload compatibility
fn value_to_musical_work_data(work: &Value) -> Option<MusicalWorkData> {
    Some(MusicalWorkData {
        id: work.get("id")?.as_str()?.to_string(),
        title: work.get("title")?.as_str()?.to_string(),
        iswc: work.get("iswc").and_then(|v| v.as_str()).map(String::from),
        creators: work.get("creators")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|c| {
                        Some(CreatorData {
                            name: String::new(), // Not displayed anymore
                            roles: c.get("role")
                                .and_then(|r| r.as_str())
                                .map(|s| vec![s.to_string()])
                                .unwrap_or_default(),
                            id: c.get("id")?.clone(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
        creation_year: work.get("creationYear").and_then(|v| v.as_u64()).map(|y| y as u16),
        is_instrumental: work.get("instrumental").and_then(|v| v.as_bool()).unwrap_or(false),
        work_type: work.get("workType")
            .and_then(|w| {
                // Handle both { "type": "Original" } and "Original" formats
                w.get("type").and_then(|t| t.as_str())
                    .or_else(|| w.as_str())
            })
            .unwrap_or("Unknown")
            .to_string(),
        language: work.get("language").and_then(|v| v.as_str()).map(String::from),
        musical_key: work.get("key").and_then(|v| v.as_str()).map(String::from),
    })
}

/// Display a single creator with roles
#[component]
fn CreatorItem(
    #[prop(into)] index: usize,
    #[prop(into)] creator: CreatorData,
) -> impl IntoView {
    let id_display = format_party_id(&creator.id);
    
    view! {
        <div class="midds-array-item">
            <span class="midds-index">{index}</span>
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
    let creators_count = work.creators.len();
    
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
            
            // Creators
            <div class="midds-field">
                <div class="midds-label">
                    "creators" 
                    <span class="midds-count">"(" {creators_count} ")"</span>
                </div>
                <div class="midds-array">
                    <For
                        each=move || work.creators.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(index, creator)| {
                            view! { <CreatorItem index=index creator=creator/> }
                        }
                    />
                </div>
            </div>
            
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

