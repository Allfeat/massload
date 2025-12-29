//! Release MIDDS display component

use leptos::*;
use crate::services::explorer::ReleaseData;
use super::common::{MiddsHeader, MiddsField};

/// Display a single recording reference
#[component]
fn RecordingReference(
    #[prop(into)] index: usize,
    #[prop(into)] recording_id: String,
) -> impl IntoView {
    view! {
        <div class="midds-array-item">
            <span class="midds-index">{index}</span>
            <span class="reference-id">{recording_id}</span>
        </div>
    }
}

/// Display release in MIDDS format
#[component]
pub fn ReleaseDisplay(
    #[prop(into)] release: ReleaseData,
) -> impl IntoView {
    let recordings_count = release.recording_ids.len();
    
    view! {
        <div class="midds-release">
            <MiddsHeader midds_type="Release"/>
            
            // Title
            <MiddsField label="title">
                {release.title}
            </MiddsField>
            
            // UPC (optional)
            {release.upc.map(|upc| view! {
                <MiddsField label="upc" class="upc-value">
                    {upc}
                </MiddsField>
            })}
            
            // Release Type
            <MiddsField label="releaseType" class="type-value">
                {release.release_type}
            </MiddsField>
            
            // Release Date (optional)
            {release.release_date.map(|date| view! {
                <MiddsField label="releaseDate">
                    {date}
                </MiddsField>
            })}
            
            // Label (optional)
            {release.label.map(|label| view! {
                <MiddsField label="label">
                    {label}
                </MiddsField>
            })}
            
            // Catalog Number (optional)
            {release.catalog_number.map(|cat| view! {
                <MiddsField label="catalogNumber">
                    {cat}
                </MiddsField>
            })}
            
            // Recording References
            <div class="midds-field">
                <div class="midds-label">
                    "recordings" 
                    <span class="midds-count">"(" {recordings_count} ")"</span>
                </div>
                <div class="midds-array">
                    <For
                        each=move || release.recording_ids.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(index, rec_id)| {
                            view! { <RecordingReference index=index recording_id=rec_id/> }
                        }
                    />
                </div>
            </div>
            
            // Total Tracks (optional)
            {release.total_tracks.map(|total| view! {
                <MiddsField label="totalTracks">
                    {total}
                </MiddsField>
            })}
        </div>
    }
}

