//! Hero section component

use leptos::*;
use crate::i18n::t;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="hero">
            <h1>{move || t("common.title")}</h1>
            <p class="subtitle">
                {move || t("common.subtitle")}
            </p>
        </div>
    }
}
