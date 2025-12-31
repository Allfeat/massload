//! Error dialog component with Allfeat styling.

use leptos::*;
use allfeat_ui::components::IconAlertTriangle;
use crate::i18n::t;

#[component]
pub fn ErrorDialog(
    #[prop(into)] show: ReadSignal<bool>,
    #[prop(into)] title: ReadSignal<String>,
    #[prop(into)] message: ReadSignal<String>,
    #[prop(into)] on_close: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=move || show.get()>
            <div class="modal-overlay" on:click=move |_| on_close.set(false)>
                <div class="modal-content error-modal" on:click=move |e| {
                    e.stop_propagation();
                }>
                    <div class="error-icon">
                        <IconAlertTriangle/>
                    </div>
                    <h3 class="error-title">{move || title.get()}</h3>
                    <p class="error-message">{move || message.get()}</p>
                    <div class="error-actions">
                        <button class="btn btn-primary" on:click=move |_| on_close.set(false)>
                            {move || t("common.ok")}
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}

