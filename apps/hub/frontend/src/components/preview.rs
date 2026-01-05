//! Composant Preview pour afficher et approuver les transactions

use leptos::*;
use crate::{PreviewItem, LogEntry, LogLevel};
use crate::services::BlockchainService;
use crate::i18n::t;
use crate::components::midds_display::WorkDisplayJson;
use crate::network::use_network;
use allfeat_ui::components::IconCheckCircle;

/// Available page sizes for pagination
const PAGE_SIZES: [usize; 4] = [10, 25, 50, 100];

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
    // Get current network
    let network = use_network();
    
    // Pagination state
    let (current_page, set_current_page) = create_signal(1usize);
    let (items_per_page, set_items_per_page) = create_signal(10usize);
    
    // État pour tracker quel item est expanded
    let (expanded_index, set_expanded_index) = create_signal(None::<usize>);
    
    // Confirmation dialog state
    let (show_confirm_dialog, set_show_confirm_dialog) = create_signal(false);
    
    // Success state
    let (registration_result, set_registration_result) = create_signal(None::<RegistrationResult>);
    
    // Calculate total pages
    let total_pages = move || {
        let total = data.get().map(|d| d.len()).unwrap_or(0);
        let per_page = items_per_page.get();
        if total == 0 { 1 } else { (total + per_page - 1) / per_page }
    };
    
    // Get paginated data
    let paginated_data = move || {
        data.get().map(|items| {
            let start = (current_page.get() - 1) * items_per_page.get();
            let end = std::cmp::min(start + items_per_page.get(), items.len());
            items[start..end].to_vec()
        }).unwrap_or_default()
    };
    
    // Calculate start index for display
    let start_index = move || (current_page.get() - 1) * items_per_page.get();
    
    // Handler pour annuler - reset tout et revient à la zone de drop
    let on_cancel = move || {
        log::info!("Annulation - retour à la zone de drop");
        if let Some(setter) = set_preview_data {
            setter.set(None);
        }
        if let Some(setter) = set_musical_works_json {
            setter.set(None);
        }
        set_logs.set(vec![]);
        set_is_processing.set(false);
        set_registration_result.set(None);
    };
    
    // Handler pour ouvrir le dialogue de confirmation
    let on_submit_click = move |_| {
        if wallet_connected.get() {
            set_show_confirm_dialog.set(true);
        }
    };
    
    // Handler pour confirmer et envoyer
    let on_confirm_send = move || {
        set_show_confirm_dialog.set(false);
        
        let works = musical_works_json.get();
        let address = wallet_address.get();
        
        if let Some(works_json) = works {
            // Capture current network RPC URL
            let current_network = network.get();
            let rpc_url = current_network.rpc_url().to_string();
            
            log::info!("Envoi des transactions...");
            log::info!("Using network: {} ({})", current_network.name(), rpc_url);
            set_is_processing.set(true);
            
            let works_count = works_json.as_array().map(|a| a.len()).unwrap_or(0);
            
            set_logs.update(|logs| {
                logs.push(LogEntry {
                    level: LogLevel::Info,
                    message: t("preview.sending_works").replace("{count}", &works_count.to_string()),
                    timestamp: js_sys::Date::new_0().to_locale_time_string("fr-FR").as_string().unwrap_or_default(),
                });
            });
            
            spawn_local(async move {
                let blockchain = BlockchainService::with_rpc_url(rpc_url);
                
                match blockchain.submit_works(works_json.clone(), address).await {
                    Ok(result) => {
                        if result.success {
                            let success_count = result.work_results.iter().filter(|w| w.success).count();
                            log::info!("Transaction confirmée: {:?}", result.tx_hash);
                            
                            // Set success result
                            set_registration_result.set(Some(RegistrationResult {
                                success: true,
                                tx_hash: result.tx_hash.clone(),
                                error: None,
                            }));
                            
                            set_logs.update(|logs| {
                                logs.push(LogEntry {
                                    level: LogLevel::Success,
                                    message: format!("{}/{} {} {}", 
                                        success_count, works_count,
                                        t("preview.works_registered"),
                                        result.tx_hash.as_deref().unwrap_or("?")),
                                    timestamp: js_sys::Date::new_0().to_locale_time_string("fr-FR").as_string().unwrap_or_default(),
                                });
                            });
                        } else {
                            log::error!("Transaction échouée: {:?}", result.error);
                            set_logs.update(|logs| {
                                logs.push(LogEntry {
                                    level: LogLevel::Error,
                                    message: format!("{}: {}", t("common.error"), result.error.as_deref().unwrap_or("Erreur inconnue")),
                                    timestamp: js_sys::Date::new_0().to_locale_time_string("fr-FR").as_string().unwrap_or_default(),
                                });
                            });
                        }
                    }
                    Err(e) => {
                        log::error!("Erreur blockchain: {}", e);
                        set_logs.update(|logs| {
                            logs.push(LogEntry {
                                level: LogLevel::Error,
                                message: format!("{}: {}", t("common.error"), e),
                                timestamp: js_sys::Date::new_0().to_locale_time_string("fr-FR").as_string().unwrap_or_default(),
                            });
                        });
                    }
                }
                
                set_is_processing.set(false);
            });
        }
    };
    
    // Pagination handlers
    let on_prev_page = move |_| {
        if current_page.get() > 1 {
            set_current_page.update(|p| *p -= 1);
            set_expanded_index.set(None);
        }
    };
    
    let on_next_page = move |_| {
        if current_page.get() < total_pages() {
            set_current_page.update(|p| *p += 1);
            set_expanded_index.set(None);
        }
    };
    
    let on_page_size_change = move |ev| {
        let target = event_target::<web_sys::HtmlSelectElement>(&ev);
        if let Ok(size) = target.value().parse::<usize>() {
            set_items_per_page.set(size);
            set_current_page.set(1);
            set_expanded_index.set(None);
        }
    };
    
    view! {
        // Success message overlay
        <Show
            when=move || registration_result.get().map(|r| r.success).unwrap_or(false)
            fallback=|| view! { }
        >
            <SuccessMessage 
                result=registration_result 
                on_close=on_cancel
            />
        </Show>
        
        // Confirmation dialog
        <Show
            when=move || show_confirm_dialog.get()
            fallback=|| view! { }
        >
            <ConfirmDialog
                works_count=move || data.get().map(|d| d.len()).unwrap_or(0)
                on_confirm=on_confirm_send
                on_cancel=move || set_show_confirm_dialog.set(false)
            />
        </Show>
        
        <div class="preview-section show">
            <div id="previewHeader" class="preview-header">
                <h2>{move || t("preview.title")}</h2>
                <div class="preview-stats">
                    <span class="stat-item">
                        {move || t("preview.total_works")} ": "
                        <strong>{move || data.get().map(|d| d.len()).unwrap_or(0)}</strong>
                    </span>
                    <span class="stat-divider">"•"</span>
                    <span class="stat-item">
                        {move || t("preview.estimated_cost")} ": "
                        <strong>
                            {move || {
                                let count = data.get().map(|d| d.len()).unwrap_or(0);
                                format!("{:.2} AFT", count as f64 * 0.05)
                            }}
                        </strong>
                    </span>
                </div>
            </div>
            
            // Pagination controls (top)
            <div class="pagination-controls pagination-top">
                <div class="pagination-info">
                    {move || t("preview.page")} " "
                    <strong>{move || current_page.get()}</strong>
                    " " {move || t("preview.of")} " "
                    <strong>{move || total_pages()}</strong>
                    " (" {move || data.get().map(|d| d.len()).unwrap_or(0)} " " {move || t("preview.works_count")} ")"
                </div>
                
                <div class="pagination-size">
                    <label for="pageSize">{move || t("preview.items_per_page")} ": "</label>
                    <select id="pageSize" on:change=on_page_size_change>
                        {PAGE_SIZES.iter().map(|&size| {
                            view! {
                                <option 
                                    value=size.to_string()
                                    selected=move || items_per_page.get() == size
                                >
                                    {size}
                                </option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
                
                <div class="pagination-buttons">
                    <button 
                        class="btn-pagination"
                        on:click=on_prev_page
                        disabled=move || { current_page.get() <= 1 }
                    >
                        "◀ " {move || t("preview.previous")}
                    </button>
                    <button 
                        class="btn-pagination"
                        on:click=on_next_page
                        disabled=move || { current_page.get() >= total_pages() }
                    >
                        {move || t("preview.next")} " ▶"
                    </button>
                </div>
            </div>
            
            <div id="previewContent">
                <div class="preview-list" id="previewList">
                    <For
                        each=move || {
                            let start = start_index();
                            paginated_data().into_iter().enumerate().map(move |(idx, item)| (start + idx, item)).collect::<Vec<_>>()
                        }
                        key=|(global_idx, _)| *global_idx
                        children=move |(global_idx, item)| {
                            let is_expanded = move || expanded_index.get() == Some(global_idx);
                            
                            let toggle_expand = move |_| {
                                if expanded_index.get() == Some(global_idx) {
                                    set_expanded_index.set(None);
                                } else {
                                    set_expanded_index.set(Some(global_idx));
                                }
                            };
                            
                            // Get the full JSON for this item
                            let full_work_json = move || {
                                musical_works_json.get()
                                    .and_then(|json| {
                                        json.as_array()
                                            .and_then(|arr| arr.get(global_idx).cloned())
                                    })
                            };
                            
                            view! {
                                <div class="preview-item" class:expanded=is_expanded>
                                    <div class="preview-item-header" on:click=toggle_expand style="cursor: pointer;">
                                        <div class="preview-item-number">{global_idx + 1}</div>
                                        <div class="preview-item-content">
                                            <div class="preview-item-title">
                                                {if is_expanded() { "▼ " } else { "▶ " }}
                                                {item.title.clone()}
                                            </div>
                                            <div class="preview-item-details">
                                                {move || t("preview.iswc")} ": " {item.iswc.clone()} " • " 
                                                {move || t("preview.creators")} ": " {item.creators_count}
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <Show
                                        when=is_expanded
                                        fallback=|| view! { }
                                    >
                                        <div class="preview-item-expanded">
                                            {move || {
                                                if let Some(work_json) = full_work_json() {
                                                    view! {
                                                        <WorkDisplayJson work=work_json/>
                                                    }.into_view()
                                                } else {
                                                    view! { <p>{move || t("preview.invalid_work_data")}</p> }.into_view()
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
            
            // Pagination controls (bottom)
            <div class="pagination-controls pagination-bottom">
                <div class="pagination-info">
                    {move || t("preview.page")} " "
                    <strong>{move || current_page.get()}</strong>
                    " " {move || t("preview.of")} " "
                    <strong>{move || total_pages()}</strong>
                </div>
                
                <div class="pagination-buttons">
                    <button 
                        class="btn-pagination"
                        on:click=on_prev_page
                        disabled=move || { current_page.get() <= 1 }
                    >
                        "◀ " {move || t("preview.previous")}
                    </button>
                    <button 
                        class="btn-pagination"
                        on:click=on_next_page
                        disabled=move || { current_page.get() >= total_pages() }
                    >
                        {move || t("preview.next")} " ▶"
                    </button>
                </div>
            </div>
            
            <div id="previewFooter" class="preview-footer">
                <button 
                    class="btn btn-secondary"
                    on:click=move |_| on_cancel()
                >
                    {move || t("preview.cancel")}
                </button>
                <button 
                    class="btn btn-primary"
                    on:click=on_submit_click
                    disabled=move || !wallet_connected.get()
                >
                    {move || t("preview.register_on_chain")}
                </button>
            </div>
        </div>
    }
}

/// Registration result data
#[derive(Clone, Debug)]
struct RegistrationResult {
    success: bool,
    tx_hash: Option<String>,
    #[allow(dead_code)]
    error: Option<String>,
}

/// Confirmation dialog component
#[component]
fn ConfirmDialog(
    works_count: impl Fn() -> usize + 'static,
    on_confirm: impl Fn() + 'static,
    on_cancel: impl Fn() + Copy + 'static,
) -> impl IntoView {
    let _ = works_count; // Keep for future use
    
    view! {
        <div class="modal-overlay">
            <div class="modal-content confirm-dialog">
                <button 
                    class="modal-close-button" 
                    on:click=move |_| on_cancel()
                    aria-label="Close"
                >
                    "×"
                </button>
                <h2>{move || t("preview.confirm_title")}</h2>
                <p class="confirm-message">
                    {move || t("preview.confirm_message")}
                </p>
                <div class="confirm-buttons">
                    <button class="btn btn-secondary" on:click=move |_| on_cancel()>
                        {move || t("preview.cancel")}
                    </button>
                    <button class="btn btn-primary" on:click=move |_| on_confirm()>
                        {move || t("preview.confirm")}
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Success message component
#[component]
fn SuccessMessage(
    result: ReadSignal<Option<RegistrationResult>>,
    on_close: impl Fn() + Copy + 'static,
) -> impl IntoView {
    view! {
        <div class="modal-overlay success-overlay">
            <div class="modal-content success-message">
                <div class="success-icon"><IconCheckCircle/></div>
                <h2>{move || t("preview.success_title")}</h2>
                
                <div class="success-illustration">
                    <svg width="180" height="140" viewBox="0 0 180 140" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <rect x="40" y="20" width="100" height="80" rx="8" fill="var(--color-primary)" opacity="0.15"/>
                        <rect x="50" y="30" width="80" height="60" rx="6" fill="var(--color-primary)" opacity="0.25"/>
                        <rect x="60" y="40" width="60" height="40" rx="4" fill="var(--color-primary)" opacity="0.35"/>
                        <path d="M75 60L85 70L105 50" stroke="var(--color-primary)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                </div>
                
                <p class="success-description">
                    {move || t("preview.success_message")}
                </p>
                
                <p class="success-immutable">
                    {move || t("preview.success_immutable")}
                </p>
                
                <Show
                    when=move || result.get().and_then(|r| r.tx_hash.clone()).is_some()
                    fallback=|| view! { }
                >
                    <div class="tx-hash">
                        <span class="tx-label">{move || t("preview.tx_hash")}</span>
                        <code class="tx-value">
                            {move || result.get().and_then(|r| r.tx_hash.clone()).unwrap_or_default()}
                        </code>
                    </div>
                </Show>
                
                <button class="btn btn-primary" on:click=move |_| on_close()>
                    {move || t("preview.back_to_upload")}
                </button>
            </div>
        </div>
    }
}
