//! Composant Preview pour afficher et approuver les transactions

use leptos::*;
use crate::{PreviewItem, WorkDetail, LogEntry, LogLevel};
use crate::services::BlockchainService;

#[component]
pub fn PreviewSection(
    data: ReadSignal<Option<Vec<PreviewItem>>>,
    musical_works_json: ReadSignal<Option<serde_json::Value>>,
    wallet_connected: ReadSignal<bool>,
    wallet_address: ReadSignal<Option<String>>,
    set_logs: WriteSignal<Vec<crate::LogEntry>>,
    set_is_processing: WriteSignal<bool>,
    #[prop(optional)] set_preview_data: Option<WriteSignal<Option<Vec<PreviewItem>>>>,
    #[prop(optional)] set_musical_works_json: Option<WriteSignal<Option<serde_json::Value>>>,
) -> impl IntoView {
    // État pour tracker quel item est expanded
    let (expanded_index, set_expanded_index) = create_signal(None::<usize>);
    
    // Handler pour annuler - reset tout et revient à la zone de drop
    let on_cancel = move |_| {
        log::info!("🚫 Annulation - retour à la zone de drop");
        // Clear preview data
        if let Some(setter) = set_preview_data {
            setter.set(None);
        }
        if let Some(setter) = set_musical_works_json {
            setter.set(None);
        }
        // Clear logs to show upload box again
        set_logs.set(vec![]);
        set_is_processing.set(false);
    };
    
    // Handler pour signer et envoyer
    let on_sign_and_send = move |_| {
        let works = musical_works_json.get();
        let connected = wallet_connected.get();
        let address = wallet_address.get();
        
        if !connected {
            log::warn!("⚠️ Wallet non connecté");
            set_logs.update(|logs| {
                logs.push(LogEntry {
                    level: LogLevel::Warning,
                    message: "Veuillez connecter votre wallet avant de signer".to_string(),
                    timestamp: js_sys::Date::new_0().to_locale_time_string("fr-FR").as_string().unwrap_or_default(),
                });
            });
            return;
        }
        
        if let Some(works_json) = works {
            log::info!("📤 Envoi des transactions...");
            set_is_processing.set(true);
            
            set_logs.update(|logs| {
                logs.push(LogEntry {
                    level: LogLevel::Info,
                    message: format!("Envoi de {} œuvres à la blockchain...", 
                        works_json.as_array().map(|a| a.len()).unwrap_or(0)),
                    timestamp: js_sys::Date::new_0().to_locale_time_string("fr-FR").as_string().unwrap_or_default(),
                });
            });
            
            // Spawn async task pour envoyer à la blockchain
            spawn_local(async move {
                let blockchain = BlockchainService::new();
                
                match blockchain.submit_works(works_json.clone(), address).await {
                    Ok(result) => {
                        if result.success {
                            let success_count = result.work_results.iter().filter(|w| w.success).count();
                            let total = result.work_results.len();
                            log::info!("✅ Transaction confirmée: {:?}", result.tx_hash);
                        set_logs.update(|logs| {
                            logs.push(LogEntry {
                                level: LogLevel::Success,
                                    message: format!("✅ {}/{} œuvres enregistrées! Hash: {}", 
                                        success_count, total,
                                        result.tx_hash.as_deref().unwrap_or("?")),
                                timestamp: js_sys::Date::new_0().to_locale_time_string("fr-FR").as_string().unwrap_or_default(),
                            });
                        });
                        } else {
                            log::error!("❌ Transaction échouée: {:?}", result.error);
                            set_logs.update(|logs| {
                                logs.push(LogEntry {
                                    level: LogLevel::Error,
                                    message: format!("❌ Échec: {}", result.error.as_deref().unwrap_or("Erreur inconnue")),
                                    timestamp: js_sys::Date::new_0().to_locale_time_string("fr-FR").as_string().unwrap_or_default(),
                                });
                            });
                        }
                    }
                    Err(e) => {
                        log::error!("❌ Erreur blockchain: {}", e);
                        set_logs.update(|logs| {
                            logs.push(LogEntry {
                                level: LogLevel::Error,
                                message: format!("❌ Erreur: {}", e),
                                timestamp: js_sys::Date::new_0().to_locale_time_string("fr-FR").as_string().unwrap_or_default(),
                            });
                        });
                    }
                }
                
                set_is_processing.set(false);
            });
        }
    };
    
    view! {
        <div class="preview-section show" id="previewSection">
            <div class="preview-header">
                <div class="preview-title">"📋 Aperçu des transactions"</div>
                <button class="btn btn-secondary" id="cancelBtn" on:click=on_cancel>"Annuler"</button>
            </div>
            
            <div id="previewContent">
                <div class="preview-list" id="previewList">
                    <For
                        each=move || data.get().unwrap_or_default().into_iter().enumerate()
                        key=|(idx, _)| *idx
                        children=move |(idx, item)| {
                            let is_expanded = move || expanded_index.get() == Some(idx);
                            
                            let toggle_expand = move |_| {
                                if expanded_index.get() == Some(idx) {
                                    set_expanded_index.set(None);
                                } else {
                                    set_expanded_index.set(Some(idx));
                                }
                            };
                            
                            // Récupérer le JSON complet pour cet item
                            let full_work = move || {
                                musical_works_json.get()
                                    .and_then(|json| {
                                        json.as_array()
                                            .and_then(|arr| arr.get(idx).cloned())
                                    })
                            };
                            
                            view! {
                                <div class="preview-item" class:expanded=is_expanded>
                                    <div class="preview-item-header" on:click=toggle_expand style="cursor: pointer;">
                                        <div class="preview-item-title">
                                            {if is_expanded() { "▼ " } else { "▶ " }}
                                            {item.title.clone()}
                                        </div>
                                        <div class="preview-item-details">
                                            "ISWC: " {item.iswc.clone()} " • Créateurs: " {item.creators_count}
                                        </div>
                                    </div>
                                    
                                    <Show
                                        when=is_expanded
                                        fallback=|| view! { }
                                    >
                                        <div class="preview-item-expanded">
                                            {move || {
                                                if let Some(work) = full_work() {
                                                    view! {
                                                        <WorkDetail work=work/>
                                                    }.into_view()
                                                } else {
                                                    view! {
                                                        <div>"Détails non disponibles"</div>
                                                    }.into_view()
                                                }
                                            }}
                                        </div>
                                    </Show>
                                </div>
                            }
                        }
                    />
                </div>
            </div>

            <div class="preview-footer">
                <div class="preview-cost">
                    <strong>{move || data.get().map(|d| d.len()).unwrap_or(0)}</strong> " œuvres • "
                    "Coût estimé: " <strong>{move || format!("{:.2}", data.get().map(|d| d.len()).unwrap_or(0) as f32 * 0.05)}</strong> " AFT"
                </div>
                <button 
                    class="btn btn-primary" 
                    id="signAndSendBtn"
                    on:click=on_sign_and_send
                    disabled=move || !wallet_connected.get()
                >
                    {move || if wallet_connected.get() { "Signer & Envoyer" } else { "Connectez votre wallet" }}
                </button>
            </div>
        </div>
    }
}
