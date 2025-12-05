//! Hero section component

use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="hero">
            <h1>"Mass Load - Import CSV"</h1>
            <p class="subtitle">
                "Pour les organismes de gestion (SACEM, ASCAP, GEMA, etc.). "
                "Importez un fichier CSV pour enregistrer plusieurs œuvres musicales en batch."
            </p>
        </div>
    }
}
