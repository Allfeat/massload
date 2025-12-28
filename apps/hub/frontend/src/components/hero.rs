//! Hero section component

use leptos::*;
use crate::i18n::{use_language, Translations};

#[component]
pub fn Hero() -> impl IntoView {
    let lang = use_language();

    view! {
        <div class="hero">
            <h1>{move || Translations::title(lang.get())}</h1>
            <p class="subtitle">
                {move || Translations::subtitle(lang.get())}
            </p>
        </div>
    }
}
