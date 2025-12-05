//! Real-time log streaming using Server-Sent Events (SSE).
//!
//! Connects to the backend's `/api/logs` endpoint and displays
//! processing logs in real-time with auto-scroll support.

use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventSource, MessageEvent};

use crate::{LogEntry, LogLevel, BACKEND_URL, MAX_LOG_ENTRIES};

/// Request animation frame helper for smooth scrolling
fn request_animation_frame(f: impl FnOnce() + 'static) {
    let closure = Closure::once(f);
    web_sys::window()
        .unwrap()
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}

/// Parse SSE log entry into our LogEntry format
fn parse_sse_log(json: &str) -> Option<LogEntry> {
    let value: serde_json::Value = serde_json::from_str(json).ok()?;
    let level = match value.get("level")?.as_str()? {
        "success" => LogLevel::Success,
        "warning" => LogLevel::Warning,
        "error" => LogLevel::Error,
        _ => LogLevel::Info,
    };
    let message = value.get("message")?.as_str()?.to_string();
    
    Some(LogEntry {
        level,
        message,
        timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
    })
}

/// Start SSE connection to receive real-time logs
/// Should be called ONCE at app startup
pub fn init_sse_logs(set_logs: WriteSignal<Vec<LogEntry>>) {
    let sse_url = format!("{}/api/logs", BACKEND_URL);
    
    let event_source = match EventSource::new(&sse_url) {
        Ok(es) => es,
        Err(e) => {
            log::error!("Failed to create EventSource: {:?}", e);
            return;
        }
    };
    
    // Handle messages
    let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
        if let Some(data) = event.data().as_string() {
            if let Some(entry) = parse_sse_log(&data) {
                set_logs.update(|logs| {
                    logs.push(entry);
                    // Keep max logs in memory
                    if logs.len() > MAX_LOG_ENTRIES {
                        logs.remove(0);
                    }
                });
            }
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    
    event_source.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    
    // Handle open
    let onopen = Closure::wrap(Box::new(move |_: web_sys::Event| {
        log::info!("📡 SSE connected to logs stream");
    }) as Box<dyn FnMut(web_sys::Event)>);
    
    event_source.set_onopen(Some(onopen.as_ref().unchecked_ref()));
    onopen.forget();
    
    // Handle errors  
    let onerror = Closure::wrap(Box::new(move |_: web_sys::Event| {
        log::warn!("SSE connection error - will auto-reconnect");
    }) as Box<dyn FnMut(web_sys::Event)>);
    
    event_source.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    onerror.forget();
    
    // Store event_source to prevent it from being dropped
    // We leak it intentionally as it should live for the app's lifetime
    std::mem::forget(event_source);
    
    log::info!("📡 SSE log stream initialized");
}

/// Real-time logs panel component (display only, SSE is initialized elsewhere)
#[component]
pub fn LogsPanel(
    /// Signal for logs data
    logs: ReadSignal<Vec<LogEntry>>,
    /// Set logs signal (for clearing)
    set_logs: WriteSignal<Vec<LogEntry>>,
) -> impl IntoView {
    // Reference to the logs content div for auto-scroll
    let logs_container = create_node_ref::<leptos::html::Div>();
    
    // Auto-scroll to bottom when logs change
    create_effect(move |_| {
        // Track logs changes
        let _ = logs.get();
        
        // Scroll to bottom after DOM update
        if let Some(container) = logs_container.get() {
            // Use requestAnimationFrame to ensure DOM is updated
            request_animation_frame(move || {
                container.set_scroll_top(container.scroll_height());
            });
        }
    });
    
    view! {
        <div class="logs-panel">
            <div class="logs-header">
                <span class="logs-title">"📋 Processing Logs"</span>
                <button 
                    class="logs-clear"
                    on:click=move |_| set_logs.set(vec![])
                >
                    "Clear"
                </button>
            </div>
            <div class="logs-content" node_ref=logs_container>
                <For
                    each=move || logs.get().into_iter().enumerate()
                    key=|(i, _)| *i
                    children=move |(_, entry)| {
                        let level_class = match entry.level {
                            LogLevel::Success => "log-success",
                            LogLevel::Warning => "log-warning",
                            LogLevel::Error => "log-error",
                            _ => "log-info",
                        };
                        
                        view! {
                            <div class=format!("log-entry {}", level_class)>
                                <span class="log-time">"[" {entry.timestamp.clone()} "] "</span>
                                {entry.message.clone()}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

