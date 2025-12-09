//! Hero section component

use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="hero">
            <h1>"Mass Load"</h1>
            <p class="subtitle">
                "Enregistrement en masse d'œuvres musicales sur la blockchain Allfeat. "
                "Importez votre fichier CSV pour certifier vos métadonnées de manière décentralisée."
            </p>
        </div>
    }
}
