//! Massload page - Bulk CSV upload and registration

use leptos::*;

use crate::types::{LogEntry, PreviewItem};
use crate::components::{Hero, UploadSection, LogsPanel, PreviewSection, init_sse_logs};

/// Massload page - the original Mass Load functionality
#[component]
pub fn MassloadPage(
    wallet_connected: ReadSignal<bool>,
    wallet_address: ReadSignal<Option<String>>,
    set_wallet_connected: WriteSignal<bool>,
    set_wallet_address: WriteSignal<Option<String>>,
) -> impl IntoView {
    // Page-specific state
    let (preview_data, set_preview_data) = create_signal(None::<Vec<PreviewItem>>);
    let (musical_works_json, set_musical_works_json) = create_signal(None::<serde_json::Value>);
    let (_is_processing, set_is_processing) = create_signal(false);
    let (logs, set_logs) = create_signal(Vec::<LogEntry>::new());
    
    // Initialize SSE connection for this page
    init_sse_logs(set_logs);

    view! {
        <div class="massload-page">
            <Hero/>

            // Show UploadBox when no logs, hide when logs exist
            <Show
                when=move || logs.get().is_empty()
                fallback=|| view! { }
            >
                <UploadSection
                    wallet_connected=wallet_connected
                    set_wallet_connected=set_wallet_connected
                    set_wallet_address=set_wallet_address
                    set_preview_data=set_preview_data
                    set_musical_works_json=set_musical_works_json
                    set_is_processing=set_is_processing 
                    set_logs=set_logs
                />
            </Show>
            
            // Show LogsPanel when logs exist
            <Show
                when=move || !logs.get().is_empty()
                fallback=|| view! { }
            >
                <LogsPanel logs=logs set_logs=set_logs/>
            </Show>

            // Preview section (appears after processing)
            <Show
                when=move || preview_data.get().is_some()
                fallback=|| view! { }
            >
                <PreviewSection 
                    data=preview_data
                    musical_works_json=musical_works_json
                    wallet_connected=wallet_connected
                    wallet_address=wallet_address
                    set_logs=set_logs
                    set_is_processing=set_is_processing
                    set_preview_data=set_preview_data
                    set_musical_works_json=set_musical_works_json
                />
            </Show>
        </div>
    }
}

