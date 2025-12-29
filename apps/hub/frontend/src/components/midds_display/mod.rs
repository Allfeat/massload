//! MIDDS Display Components Module
//! 
//! Provides reusable components for displaying all MIDDS types:
//! - Musical Works
//! - Recordings
//! - Releases

pub mod common;
pub mod work;
pub mod recording;
pub mod release;

// Re-export components
pub use common::{format_party_id, MiddsHeader, MiddsField};
pub use work::WorkDisplay;
pub use recording::RecordingDisplay;
pub use release::ReleaseDisplay;

use leptos::*;
use crate::services::explorer::MiddsItem;

/// Generic MIDDS display component that dispatches to the appropriate type
#[component]
pub fn MiddsDisplay(
    #[prop(into)] item: MiddsItem,
) -> impl IntoView {
    match item {
        MiddsItem::Work(work) => {
            view! { <WorkDisplay work=work/> }.into_view()
        }
        MiddsItem::Recording(recording) => {
            view! { <RecordingDisplay recording=recording/> }.into_view()
        }
        MiddsItem::Release(release) => {
            view! { <ReleaseDisplay release=release/> }.into_view()
        }
    }
}

/// Backwards compatibility: display only a musical work
/// This is what the explorer currently uses
#[component]
pub fn MiddsWorkDisplay(
    #[prop(into)] work: crate::services::explorer::MusicalWorkData,
) -> impl IntoView {
    view! { <WorkDisplay work=work/> }
}

