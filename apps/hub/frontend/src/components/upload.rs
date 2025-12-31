//! CSV upload component with drag & drop support.
//!
//! Handles file selection, upload to backend, and result parsing.

use leptos::*;
use web_sys::{Event, HtmlInputElement};
use wasm_bindgen::JsCast;
use crate::{PreviewItem, LogEntry, LogLevel};
use crate::services::upload_csv;
use crate::i18n::t;
use crate::components::ErrorDialog;
use allfeat_ui::components::IconUpload;

const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024; // 5 MB

#[component]
pub fn UploadSection(
    wallet_connected: ReadSignal<bool>,
    set_preview_data: WriteSignal<Option<Vec<PreviewItem>>>,
    set_musical_works_json: WriteSignal<Option<serde_json::Value>>,
    set_is_processing: WriteSignal<bool>,
    set_logs: WriteSignal<Vec<LogEntry>>,
) -> impl IntoView {
    let (is_uploading, set_is_uploading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    
    // Error dialog state
    let (show_error_dialog, set_show_error_dialog) = create_signal(false);
    let (error_title, set_error_title) = create_signal(String::new());
    let (error_message, set_error_message) = create_signal(String::new());

    // Handler pour le changement de fichier
    let on_file_change = move |ev: Event| {
        let input: HtmlInputElement = event_target(&ev);
        
        if let Some(files) = input.files() {
            if files.length() > 0 {
                if let Some(file) = files.get(0) {
                    // 🔒 Vérifier que le wallet est connecté
                    if !wallet_connected.get() {
                        set_error_title.set(t("upload.error_no_wallet_title").to_string());
                        set_error_message.set(t("upload.error_no_wallet_message").to_string());
                        set_show_error_dialog.set(true);
                        
                        // Réinitialiser l'input pour permettre un nouveau drop
                        input.set_value("");
                        return;
                    }
                    
                    // 📏 Vérifier la taille du fichier (5 MB max)
                    let file_size = file.size() as u64;
                    if file_size > MAX_FILE_SIZE {
                        let size_mb = file_size as f64 / (1024.0 * 1024.0);
                        set_error_title.set(t("upload.error_file_too_large_title").to_string());
                        set_error_message.set(
                            format!("{} ({:.2} MB). {} 5 MB.", 
                                t("upload.error_file_too_large_message"),
                                size_mb,
                                t("upload.error_max_size")
                            )
                        );
                        set_show_error_dialog.set(true);
                        
                        // Réinitialiser l'input
                        input.set_value("");
                        return;
                    }
                    
                    // Réinitialiser l'état
                    set_error.set(None);
                    set_preview_data.set(None);
                    set_logs.set(Vec::new());
                    
                    // Lancer l'upload
                    spawn_local(async move {
                        set_is_uploading.set(true);
                        set_is_processing.set(true);
                        
                        // Log de début
                        add_log(set_logs, LogLevel::Info, "📤 Uploading CSV file...");
                        
                        // Upload (using relative path for unified server)
                        match upload_csv(file).await {
                            Ok(response) => {
                                add_log(
                                    set_logs,
                                    LogLevel::Success,
                                    &format!("✅ Upload successful! {} works found", response.metadata.total_works),
                                );
                                
                                if response.metadata.cached {
                                    let id = response.metadata.matrix_id.as_deref().unwrap_or("unknown");
                                    add_log(
                                        set_logs,
                                        LogLevel::Info,
                                        &format!("♻️  Used cached transformation matrix: {}", id),
                                    );
                                } else {
                                    let id = response.metadata.matrix_id.as_deref().unwrap_or("new");
                                    add_log(
                                        set_logs,
                                        LogLevel::Info,
                                        &format!("🤖 AI generated new transformation matrix: {}", id),
                                    );
                                }
                                
                                // Convertir en PreviewItems
                                // Sauvegarder les musical works JSON complets
                                set_musical_works_json.set(Some(serde_json::Value::Array(response.musical_works.clone())));
                                
                                // Convertir en PreviewItems
                                let preview_items: Vec<PreviewItem> = response
                                    .musical_works
                                    .iter()
                                    .filter_map(|work| {
                                        // ISWC
                                        let iswc = work.get("iswc")?.as_str()?.to_string();
                                        
                                        // Title peut être String ou Object {title: "...", language: "..."}
                                        let title = if let Some(title_str) = work.get("title").and_then(|t| t.as_str()) {
                                            title_str.to_string()
                                        } else if let Some(title_obj) = work.get("title").and_then(|t| t.as_object()) {
                                            title_obj.get("title")?.as_str()?.to_string()
                                        } else {
                                            return None;
                                        };
                                        
                                        // Creators count
                                        let creators_count = work.get("creators")
                                            .and_then(|c| c.as_array())
                                            .map(|arr| arr.len())
                                            .unwrap_or(0);
                                        
                                        Some(PreviewItem {
                                            title,
                                            iswc,
                                            creators_count,
                                        })
                                    })
                                    .collect();
                                
                                set_preview_data.set(Some(preview_items));
                                
                                add_log(
                                    set_logs,
                                    LogLevel::Success,
                                    &format!("🎵 Estimated cost: {}", response.metadata.estimated_cost),
                                );
                            }
                            Err(e) => {
                                add_log(set_logs, LogLevel::Error, &format!("❌ Upload failed: {}", e));
                                set_error.set(Some(e));
                            }
                        }
                        
                        set_is_uploading.set(false);
                        set_is_processing.set(false);
                    });
                }
            }
        }
    };

    // Handler pour cliquer sur la zone entière
    let trigger_file_input = move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(input) = document.get_element_by_id("fileInput") {
                    if let Some(html_input) = input.dyn_ref::<HtmlInputElement>() {
                        html_input.click();
                    }
                }
            }
        }
    };

    view! {
        <div 
            class="upload-section" 
            id="uploadZone"
            on:click=trigger_file_input
        >
            <div class="upload-icon"><IconUpload/></div>
            <div class="upload-text">
                {move || if is_uploading.get() {
                    t("upload.uploading")
                } else {
                    t("common.drag_csv_here")
                }}
            </div>
            
            <Show
                when=move || !is_uploading.get()
                fallback=|| view! { }
            >
                <div class="upload-hint">{move || t("common.or_click_to_select")}</div>
                <div class="upload-hint mt-20">
                    {move || t("common.supported_formats")}
                    <br/>
                    {move || t("common.auto_transform_ai")}
                </div>
            </Show>
            
            <Show
                when=move || error.get().is_some()
                fallback=|| view! { }
            >
                <div class="error-message">
                    {move || error.get().unwrap_or_default()}
                </div>
            </Show>
            
            <input
                type="file"
                id="fileInput"
                accept=".csv"
                style="display:none"
                on:change=on_file_change
            />
            
            <Show
                when=move || !is_uploading.get()
                fallback=|| view! { }
            >
                <label for="fileInput" class="upload-button">
                    {move || t("common.choose_csv_file")}
                </label>
            </Show>
        </div>
        
        // Error Dialog
        <ErrorDialog
            show=show_error_dialog
            title=error_title
            message=error_message
            on_close=set_show_error_dialog
        />
    }
}

fn add_log(set_logs: WriteSignal<Vec<LogEntry>>, level: LogLevel, message: &str) {
    // Utiliser Date JS pour le timestamp
    let timestamp = js_sys::Date::new_0()
        .to_locale_time_string("fr-FR")
        .as_string()
        .unwrap_or_else(|| "00:00:00".to_string());
    
    set_logs.update(|logs| {
        logs.push(LogEntry {
            level,
            message: message.to_string(),
            timestamp,
        });
    });
    
    // Log aussi dans la console
    log::info!("{}", message);
}
