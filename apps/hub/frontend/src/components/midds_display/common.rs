//! Common utilities for MIDDS display components

use leptos::*;

/// Format Party ID (IPI/ISNI/Both) from JSON value
pub fn format_party_id(id: &serde_json::Value) -> String {
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

/// MIDDS header with badge and type
#[component]
pub fn MiddsHeader(
    #[prop(into)] midds_type: String,
) -> impl IntoView {
    view! {
        <div class="midds-header">
            <span class="midds-badge">"MIDDS"</span>
            <span class="midds-type">{midds_type}</span>
        </div>
    }
}

/// MIDDS field display
#[component]
pub fn MiddsField(
    #[prop(into)] label: String,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="midds-field">
            <div class="midds-label">{label}</div>
            <div class=format!("midds-value {}", class.unwrap_or_default())>
                {children()}
            </div>
        </div>
    }
}

