//! Footer component

use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div>"Copyright © 2025 Allfeat • Powered by " <span class="rust-badge">"🦀 Rust + Leptos"</span></div>
            <div class="footer-links">
                <a href="https://t.me/Allfeat_fndn" class="footer-link" target="_blank">
                    "Telegram"
                </a>
                <a href="https://www.instagram.com/allfeat/" class="footer-link" target="_blank">
                    "Instagram"
                </a>
                <a href="https://github.com/allfeat" class="footer-link" target="_blank">
                    "GitHub"
                </a>
            </div>
        </footer>
    }
}
