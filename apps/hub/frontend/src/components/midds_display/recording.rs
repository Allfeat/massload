//! Recording MIDDS display component

use leptos::*;
use crate::services::explorer::{RecordingData, PerformerData};
use super::common::{format_party_id, MiddsHeader, MiddsField};

/// Display a single performer
#[component]
fn PerformerItem(
    #[prop(into)] index: usize,
    #[prop(into)] performer: PerformerData,
) -> impl IntoView {
    let id_display = format_party_id(&performer.id);
    
    view! {
        <div class="midds-array-item">
            <span class="midds-index">{index}</span>
            <div class="midds-object">
                <div class="midds-prop">
                    <span class="prop-key">"id"</span>
                    <span class="prop-value ipi-code">{id_display}</span>
                </div>
            </div>
        </div>
    }
}

/// Display recording in MIDDS format
#[component]
pub fn RecordingDisplay(
    #[prop(into)] recording: RecordingData,
) -> impl IntoView {
    let performers_count = recording.performers.len();
    
    view! {
        <div class="midds-recording">
            <MiddsHeader midds_type="Recording"/>
            
            // Title
            <MiddsField label="title">
                {recording.title}
            </MiddsField>
            
            // ISRC (optional)
            {recording.isrc.map(|isrc| view! {
                <MiddsField label="isrc" class="isrc-value">
                    {isrc}
                </MiddsField>
            })}
            
            // Musical Work Reference
            <MiddsField label="musicalWorkId" class="reference-id">
                {recording.musical_work_id}
            </MiddsField>
            
            // Performers
            <div class="midds-field">
                <div class="midds-label">
                    "performers" 
                    <span class="midds-count">"(" {performers_count} ")"</span>
                </div>
                <div class="midds-array">
                    <For
                        each=move || recording.performers.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(index, performer)| {
                            view! { <PerformerItem index=index performer=performer/> }
                        }
                    />
                </div>
            </div>
            
            // Duration (optional)
            {recording.duration_ms.map(|dur| {
                let seconds = dur / 1000;
                let minutes = seconds / 60;
                let secs = seconds % 60;
                view! {
                    <MiddsField label="duration">
                        {format!("{}:{:02} ({} ms)", minutes, secs, dur)}
                    </MiddsField>
                }
            })}
            
            // Recording Date (optional)
            {recording.recording_date.map(|date| view! {
                <MiddsField label="recordingDate">
                    {date}
                </MiddsField>
            })}
            
            // Recording Location (optional)
            {recording.recording_location.map(|location| view! {
                <MiddsField label="recordingLocation">
                    {location}
                </MiddsField>
            })}
        </div>
    }
}

