//! MIDDS Musical Work Display Component
//! 
//! Reusable component for displaying musical work data in MIDDS format.
//! Used by both Explorer and Massload pages.

use leptos::*;
use crate::services::explorer::{MusicalWorkData, CreatorData};

/// Format Party ID (IPI/ISNI) from JSON value
fn format_party_id(id: &serde_json::Value) -> String {
    // New dedot format: { "type": "Ipi", "value": 123 }
    if let Some(id_type) = id.get("type").and_then(|t| t.as_str()) {
        match id_type {
            "Ipi" => {
                let value = id.get("value").and_then(|v| v.as_i64()).unwrap_or(0);
                return format!("IPI: {}", value);
            }
            "Isni" => {
                let value = id.get("value").and_then(|v| v.as_str()).unwrap_or("—");
                return format!("ISNI: {}", value);
            }
            "Both" => {
                if let Some(both) = id.get("value") {
                    let ipi = both.get("ipi").and_then(|v| v.as_i64()).unwrap_or(0);
                    let isni = both.get("isni").and_then(|v| v.as_str()).unwrap_or("—");
                    return format!("IPI: {} / ISNI: {}", ipi, isni);
                }
            }
            _ => {}
        }
    }
    
    // Legacy format: { "Ipi": 123 }
    if let Some(ipi) = id.get("Ipi").and_then(|v| v.as_i64()) {
        format!("IPI: {}", ipi)
    } else if let Some(isni) = id.get("Isni").and_then(|v| v.as_str()) {
        format!("ISNI: {}", isni)
    } else if let Some(both) = id.get("Both") {
        let ipi = both.get("ipi").and_then(|v| v.as_i64()).unwrap_or(0);
        let isni = both.get("isni").and_then(|v| v.as_str()).unwrap_or("—");
        format!("IPI: {} / ISNI: {}", ipi, isni)
    } else {
        "—".to_string()
    }
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
                    <span class="prop-key">"name"</span>
                    <span class="prop-value">{creator.name}</span>
                </div>
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

/// Display musical work in MIDDS format
#[component]
pub fn MiddsDisplay(
    #[prop(into)] work: MusicalWorkData,
) -> impl IntoView {
    let creators_count = work.creators.len();
    
    view! {
        <div class="midds-work">
            <div class="midds-header">
                <span class="midds-badge">"MIDDS"</span>
                <span class="midds-type">"MusicalWork"</span>
            </div>
            
            // Title
            <div class="midds-field">
                <div class="midds-label">"title"</div>
                <div class="midds-value">{work.title}</div>
            </div>
            
            // ISWC (optional)
            {work.iswc.map(|iswc| view! {
                <div class="midds-field">
                    <div class="midds-label">"iswc"</div>
                    <div class="midds-value iswc-value">{iswc}</div>
                </div>
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
                <div class="midds-field">
                    <div class="midds-label">"creationYear"</div>
                    <div class="midds-value">{year}</div>
                </div>
            })}
            
            // Work Type
            <div class="midds-field">
                <div class="midds-label">"workType"</div>
                <div class="midds-value type-value">{work.work_type}</div>
            </div>
            
            // Instrumental
            <div class="midds-field">
                <div class="midds-label">"instrumental"</div>
                <div class="midds-value bool-value">{if work.is_instrumental { "true" } else { "false" }}</div>
            </div>
            
            // Language (optional)
            {work.language.map(|lang| view! {
                <div class="midds-field">
                    <div class="midds-label">"language"</div>
                    <div class="midds-value">
                        {lang.clone()}
                        <span class="midds-lang">{lang}</span>
                    </div>
                </div>
            })}
            
            // Musical Key (optional)
            {work.musical_key.map(|key| view! {
                <div class="midds-field">
                    <div class="midds-label">"key"</div>
                    <div class="midds-value">{key}</div>
                </div>
            })}
        </div>
    }
}

