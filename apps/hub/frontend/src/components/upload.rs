//! CSV upload component with drag & drop support.
//!
//! Handles file selection, upload to backend, and result parsing.

use leptos::*;
use leptos::ev::DragEvent;
use web_sys::{Event, HtmlInputElement};
use wasm_bindgen::JsCast;
use crate::{PreviewItem, LogEntry, LogLevel};
use crate::services::upload_csv;
use crate::i18n::t;
use crate::components::{ErrorDialog, WalletRequiredDialog, WalletModal, WalletType};
use allfeat_ui::components::IconUpload;
use crate::services::wallet::PolkadotWallet;

const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024; // 5 MB

#[component]
pub fn UploadSection(
    wallet_connected: ReadSignal<bool>,
    set_wallet_connected: WriteSignal<bool>,
    set_wallet_address: WriteSignal<Option<String>>,
    set_preview_data: WriteSignal<Option<Vec<PreviewItem>>>,
    set_musical_works_json: WriteSignal<Option<serde_json::Value>>,
    set_is_processing: WriteSignal<bool>,
    set_logs: WriteSignal<Vec<LogEntry>>,
) -> impl IntoView {
    let (is_uploading, set_is_uploading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);
    
    // File pending validation
    let (selected_file, set_selected_file) = create_signal(None::<web_sys::File>);
    let (selected_file_size, set_selected_file_size) = create_signal(0u64);
    
    // Error dialog state (for file size errors)
    let (show_error_dialog, set_show_error_dialog) = create_signal(false);
    let (error_title, set_error_title) = create_signal(String::new());
    let (error_message, set_error_message) = create_signal(String::new());
    
    // Wallet required dialog state
    let (show_wallet_required_dialog, set_show_wallet_required_dialog) = create_signal(false);
    
    // Wallet modal state
    let (wallet_modal_open, set_wallet_modal_open) = create_signal(false);

    // Handler pour ouvrir la wallet modal depuis le dialog de confirmation
    let on_connect_wallet_click = Callback::new(move |_| {
        set_wallet_modal_open.set(true);
    });

    // Handler pour fermer le wallet required dialog
    let on_wallet_required_close = Callback::new(move |_| {
        set_show_wallet_required_dialog.set(false);
    });

    // Handler pour la sélection d'un wallet
    let on_wallet_select = Callback::new(move |wallet_type: WalletType| {
        set_wallet_modal_open.set(false);
        
        let wallet_key = match wallet_type {
            WalletType::SubWallet => "subwallet-js",
            WalletType::Talisman => "talisman",
            WalletType::PolkadotJs => "polkadot-js",
        };
        
        log::info!("Attempting to connect {}...", wallet_key);
        
        spawn_local(async move {
            match PolkadotWallet::connect_specific(wallet_key).await {
                Ok(account) => {
                    log::info!("Wallet connected: {}", account.address);
                    set_wallet_connected.set(true);
                    set_wallet_address.set(Some(account.address.clone()));
                    
                    // Log success
                    add_log(set_logs, LogLevel::Success, &format!("Wallet connected: {}", account.address));
                }
                Err(e) => {
                    log::error!("Wallet connection failed: {}", e);
                    add_log(set_logs, LogLevel::Error, &format!("Wallet connection failed: {}", e));
                }
            }
        });
    });

    // Handler pour fermer la modal de wallet
    let on_wallet_modal_close = Callback::new(move |_| {
        set_wallet_modal_open.set(false);
    });

    // Handler pour le changement de fichier
    let on_file_change = move |ev: Event| {
        let input: HtmlInputElement = event_target(&ev);
        
        if let Some(files) = input.files() {
            if files.length() > 0 {
                if let Some(file) = files.get(0) {
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
                    
                    // 🔒 Vérifier si le wallet est connecté AVANT de stocker le fichier
                    if !wallet_connected.get() {
                        // Stocker temporairement le fichier
                        set_selected_file.set(Some(file.clone()));
                        set_selected_file_size.set(file_size);
                        // Ouvrir le dialog pour connecter le wallet
                        set_show_wallet_required_dialog.set(true);
                        return;
                    }
                    
                    // Wallet déjà connecté : stocker le fichier pour validation
                    set_selected_file.set(Some(file.clone()));
                    set_selected_file_size.set(file_size);
                    set_error.set(None);
                }
            }
        }
    };
    
    // Fonction helper pour exécuter la validation et l'upload
    let do_validate_and_upload = move || {
        if let Some(file) = selected_file.get() {
            // 🔒 Vérifier que le wallet est connecté AVANT l'upload
            if !wallet_connected.get() {
                set_show_wallet_required_dialog.set(true);
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
                add_log(set_logs, LogLevel::Info, "Uploading CSV file...");
                        
                        // Upload (using relative path for unified server)
                        match upload_csv(file.clone()).await {
                            Ok(response) => {
                                add_log(
                                    set_logs,
                                    LogLevel::Success,
                                    &format!("Upload successful! {} works found", response.metadata.total_works),
                                );
                                
                                if response.metadata.cached {
                                    let id = response.metadata.matrix_id.as_deref().unwrap_or("unknown");
                                    add_log(
                                        set_logs,
                                        LogLevel::Info,
                                        &format!("Used cached transformation matrix: {}", id),
                                    );
                                } else {
                                    let id = response.metadata.matrix_id.as_deref().unwrap_or("new");
                                    add_log(
                                        set_logs,
                                        LogLevel::Info,
                                        &format!("AI generated new transformation matrix: {}", id),
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
                                    &format!("Estimated cost: {}", response.metadata.estimated_cost),
                                );
                            }
                            Err(e) => {
                                add_log(set_logs, LogLevel::Error, &format!("Upload failed: {}", e));
                                set_error.set(Some(e));
                            }
                        }
                        
                        set_is_uploading.set(false);
                        set_is_processing.set(false);
                        
                        // Effacer le fichier sélectionné après l'upload
                        set_selected_file.set(None);
                        set_selected_file_size.set(0);
                    });
        }
    };
    
    // Handler pour le bouton de validation (event handler)
    let on_validate_click = move |_ev| {
        do_validate_and_upload();
    };
    
    // Handler pour supprimer le fichier sélectionné
    let on_remove_file = move |_| {
        set_selected_file.set(None);
        set_selected_file_size.set(0);
        set_error.set(None);
        
        // Réinitialiser l'input file
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(input) = document.get_element_by_id("fileInput") {
                    if let Ok(input_element) = input.dyn_into::<HtmlInputElement>() {
                        input_element.set_value("");
                    }
                }
            }
        }
    };
    
    // Handler pour drag over (empêcher le comportement par défaut)
    let on_drag_over = move |ev: DragEvent| {
        ev.prevent_default();
    };
    
    // Handler pour drag enter (optionnel - pour style visuel)
    let on_drag_enter = move |ev: DragEvent| {
        ev.prevent_default();
    };
    
    // Handler pour drag leave (optionnel - pour style visuel)
    let on_drag_leave = move |ev: DragEvent| {
        ev.prevent_default();
    };
    
    // Handler pour drop (déposer le fichier)
    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();
        
        // Caster l'événement Leptos vers web_sys::DragEvent
        let native_event: &web_sys::Event = ev.as_ref();
        if let Ok(drag_event) = native_event.dyn_ref::<web_sys::DragEvent>().ok_or(()) {
            if let Some(data_transfer) = drag_event.data_transfer() {
                if let Some(files) = data_transfer.files() {
                    if files.length() > 0 {
                        if let Some(file) = files.get(0) {
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
                                return;
                            }
                            
                            // 🔒 Vérifier si le wallet est connecté AVANT de stocker le fichier
                            if !wallet_connected.get() {
                                // Stocker temporairement le fichier
                                set_selected_file.set(Some(file.clone()));
                                set_selected_file_size.set(file_size);
                                // Ouvrir le dialog pour connecter le wallet
                                set_show_wallet_required_dialog.set(true);
                                return;
                            }
                            
                            // Wallet déjà connecté : stocker le fichier pour validation
                            set_selected_file.set(Some(file.clone()));
                            set_selected_file_size.set(file_size);
                            set_error.set(None);
                        }
                    }
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
        // FILE SELECTION / UPLOAD ZONE
        <Show
            when=move || selected_file.get().is_none() && !is_uploading.get()
            fallback=move || view! { <></> }
        >
            <div 
                class="upload-section" 
                id="uploadZone"
                on:click=trigger_file_input
                on:dragover=on_drag_over
                on:dragenter=on_drag_enter
                on:dragleave=on_drag_leave
                on:drop=on_drop
            >
                <div class="upload-icon"><IconUpload/></div>
                <div class="upload-text">
                    {move || t("common.drag_csv_here")}
                </div>
                <div class="upload-hint">{move || t("common.or_click_to_select")}</div>
                <div class="upload-hint mt-20">
                    {move || t("common.supported_formats")}
                    <br/>
                    {move || t("common.auto_transform_ai")}
                </div>
                
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
                    accept=".csv,.xlsx,.xls"
                    style="display:none"
                    on:change=on_file_change
                />
                
                <label for="fileInput" class="upload-button">
                    {move || t("common.choose_csv_file")}
                </label>
            </div>
        </Show>
        
        // FILE VALIDATION STAGE (file selected but not validated yet)
        <Show
            when=move || selected_file.get().is_some() && !is_uploading.get()
            fallback=move || view! { <></> }
        >
            <div class="file-validation-section">
                <div class="file-info-card">
                    <div class="file-info-header">
                        <div class="file-icon-box">
                            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
                                <polyline points="14 2 14 8 20 8"></polyline>
                            </svg>
                        </div>
                        <div class="file-details">
                            <div class="file-name">
                                {move || selected_file.get().map(|f| f.name()).unwrap_or_default()}
                            </div>
                            <div class="file-size">
                                {move || {
                                    let size = selected_file_size.get();
                                    if size < 1024 {
                                        format!("{} bytes", size)
                                    } else if size < 1024 * 1024 {
                                        format!("{:.2} KB", size as f64 / 1024.0)
                                    } else {
                                        format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                    <div class="file-validation-actions">
                        <button class="btn btn-secondary" on:click=on_remove_file>
                            {move || t("common.remove")}
                        </button>
                        <button class="btn btn-primary" on:click=on_validate_click>
                            {move || t("common.validate")}
                        </button>
                    </div>
                </div>
                
                <input
                    type="file"
                    id="fileInput"
                    accept=".csv"
                    style="display:none"
                    on:change=on_file_change
                />
            </div>
        </Show>
        
        // UPLOADING STATE
        <Show
            when=move || is_uploading.get()
            fallback=move || view! { <></> }
        >
            <div class="upload-section uploading">
                <div class="upload-icon"><IconUpload/></div>
                <div class="upload-text">
                    {move || t("upload.uploading")}
                </div>
            </div>
        </Show>
        
        // Error Dialog (for file size errors)
        <ErrorDialog
            show=show_error_dialog
            title=error_title
            message=error_message
            on_close=set_show_error_dialog
        />
        
        // Wallet Required Dialog (Issue #3)
        <WalletRequiredDialog
            show=show_wallet_required_dialog
            on_connect_click=on_connect_wallet_click
            on_close=on_wallet_required_close
        />
        
        // Wallet Selection Modal (Issue #3)
        <WalletModal
            show=Signal::derive(move || wallet_modal_open.get())
            on_select=on_wallet_select
            on_close=on_wallet_modal_close
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
