use leptos::*;
use crate::{LogEntry, LogLevel};

#[component]
pub fn ProgressSection(logs: ReadSignal<Vec<LogEntry>>) -> impl IntoView {
    view! {
        <div class="progress-section show" id="progressSection">
            <div class="progress-bar">
                <div class="progress-fill" id="progressFill" style="width: 0%;"></div>
            </div>
            <div class="logs" id="logs">
                <For
                    each=move || logs.get().into_iter().enumerate()
                    key=|(idx, _)| *idx
                    children=move |(_, entry)| {
                        let class_name = match entry.level {
                            LogLevel::Info => "log-entry info",
                            LogLevel::Success => "log-entry success",
                            LogLevel::Error => "log-entry error",
                            LogLevel::Warning => "log-entry warning",
                            LogLevel::Debug => "log-entry debug",
                        };
                        view! {
                            <div class=class_name>
                                "[" {entry.timestamp} "] " {entry.message}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
