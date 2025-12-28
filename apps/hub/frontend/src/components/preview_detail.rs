//! Detailed view for a musical work - MIDDS Format

use leptos::*;
use serde_json::Value;

/// Extract ID display (IPI, ISNI, or Both)
/// Supports both formats:
/// - dedot: { "type": "Ipi", "value": 123 }
/// - legacy: { "Ipi": 123 }
fn format_party_id(id: &Value) -> String {
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

/// Creator data from MIDDS format
struct MiddsCreator {
    id_display: String,
    role: String,
}

/// Extract and display work details in MIDDS format
#[component]
pub fn WorkDetail(work: Value) -> impl IntoView {
    // === MIDDS Field Extraction ===
    
    // iswc: String
    let iswc = work.get("iswc")
        .and_then(|v| v.as_str())
        .unwrap_or("—")
        .to_string();
    
    // title: String
    let title = work.get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("—")
        .to_string();
    
    // creationYear: Number
    let creation_year = work.get("creationYear")
        .and_then(|y| y.as_i64());
    
    // instrumental: Boolean
    let instrumental = work.get("instrumental")
        .and_then(|i| i.as_bool());
    
    // language: String (enum)
    let language = work.get("language")
        .and_then(|l| l.as_str())
        .map(|s| s.to_string());
    
    // bpm: Number
    let bpm = work.get("bpm")
        .and_then(|b| b.as_i64());
    
    // key: String (enum)
    let key = work.get("key")
        .and_then(|k| k.as_str())
        .map(|s| s.to_string());
    
    // workType: { type: "Original" } | "Original" | null
    let work_type = work.get("workType")
        .and_then(|w| {
            // New dedot format: { "type": "Original" }
            if let Some(t) = w.get("type").and_then(|t| t.as_str()) {
                Some(t.to_string())
            } else {
                // Legacy format: "Original"
                w.as_str().map(|s| s.to_string())
            }
        });
    
    // creators: Array<{ id: PartyId, role: String }>
    let creators: Vec<MiddsCreator> = work.get("creators")
        .and_then(|c| c.as_array())
        .map(|arr| {
            arr.iter().map(|c| {
                let id_display = c.get("id")
                    .map(|id| format_party_id(id))
                    .unwrap_or_else(|| "—".to_string());
                let role = c.get("role")
                    .and_then(|r| r.as_str())
                    .unwrap_or("—")
                    .to_string();
                MiddsCreator { id_display, role }
            }).collect()
        })
        .unwrap_or_default();
    
    // classicalInfo: { opus?, catalogNumber?, numberOfVoices? }
    let classical_info = work.get("classicalInfo");
    let opus = classical_info
        .and_then(|ci| ci.get("opus"))
        .and_then(|o| o.as_str())
        .map(|s| s.to_string());
    let catalog_number = classical_info
        .and_then(|ci| ci.get("catalogNumber"))
        .and_then(|c| c.as_str())
        .map(|s| s.to_string());
    let number_of_voices = classical_info
        .and_then(|ci| ci.get("numberOfVoices"))
        .and_then(|n| n.as_i64());
    
    let has_classical_info = opus.is_some() || catalog_number.is_some() || number_of_voices.is_some();
    
    // === MIDDS Display ===
    view! {
        <div class="midds-work">
            // Header with MIDDS badge
            <div class="midds-header">
                <span class="midds-badge">"MIDDS"</span>
                <span class="midds-type">"MusicalWork"</span>
            </div>
            
            // iswc
            <div class="midds-field">
                <div class="midds-label">"iswc"</div>
                <div class="midds-value iswc-value">{iswc}</div>
            </div>
            
            // title
            <div class="midds-field">
                <div class="midds-label">"title"</div>
                <div class="midds-value">{title}</div>
            </div>
            
            // creationYear
            {creation_year.map(|year| view! {
                <div class="midds-field">
                    <div class="midds-label">"creationYear"</div>
                    <div class="midds-value">{year}</div>
                </div>
            })}
            
            // language
            {language.map(|lang| view! {
                <div class="midds-field">
                    <div class="midds-label">"language"</div>
                    <div class="midds-value">{lang}</div>
                </div>
            })}
            
            // instrumental
            {instrumental.map(|is_inst| view! {
                <div class="midds-field">
                    <div class="midds-label">"instrumental"</div>
                    <div class="midds-value bool-value">{if is_inst { "true" } else { "false" }}</div>
                </div>
            })}
            
            // bpm
            {bpm.map(|b| view! {
                <div class="midds-field">
                    <div class="midds-label">"bpm"</div>
                    <div class="midds-value">{b}</div>
                </div>
            })}
            
            // key
            {key.map(|k| view! {
                <div class="midds-field">
                    <div class="midds-label">"key"</div>
                    <div class="midds-value">{k}</div>
                </div>
            })}
            
            // workType
            {work_type.map(|wtype| view! {
                <div class="midds-field">
                    <div class="midds-label">"workType"</div>
                    <div class="midds-value type-value">{wtype}</div>
                </div>
            })}
            
            // creators[]
            {if !creators.is_empty() {
                view! {
                    <div class="midds-field">
                        <div class="midds-label">"creators" <span class="midds-count">"[" {creators.len()} "]"</span></div>
                        <div class="midds-array">
                            {creators.into_iter().enumerate().map(|(idx, c)| view! {
                                <div class="midds-array-item">
                                    <span class="midds-index">{idx}</span>
                                    <div class="midds-object">
                                        <div class="midds-prop">
                                            <span class="prop-key">"id"</span>
                                            <span class="prop-value ipi-code">{c.id_display}</span>
                                        </div>
                                        <div class="midds-prop">
                                            <span class="prop-key">"role"</span>
                                            <span class="prop-value role-badge">{c.role}</span>
                                        </div>
                                    </div>
                                </div>
                            }).collect_view()}
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
            
            // classicalInfo
            {if has_classical_info {
                view! {
                    <div class="midds-field">
                        <div class="midds-label">"classicalInfo"</div>
                        <div class="midds-object" style="margin-left: 1rem;">
                            {opus.map(|o| view! {
                                        <div class="midds-prop">
                                    <span class="prop-key">"opus"</span>
                                    <span class="prop-value">{o}</span>
                                        </div>
                            })}
                            {catalog_number.map(|cn| view! {
                                            <div class="midds-prop">
                                    <span class="prop-key">"catalogNumber"</span>
                                    <span class="prop-value">{cn}</span>
                                            </div>
                                        })}
                            {number_of_voices.map(|nv| view! {
                                <div class="midds-prop">
                                    <span class="prop-key">"numberOfVoices"</span>
                                    <span class="prop-value">{nv}</span>
                                </div>
                            })}
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
        </div>
    }
}
