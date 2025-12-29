//! Internationalization (i18n) module for Allfeat Hub
//! Supports: English, French, Spanish, German, Japanese, Korean, Greek

use leptos::SignalGet;

pub mod language;
pub mod header;
pub mod common;
pub mod nav;
pub mod home;
pub mod register;
pub mod protect;
pub mod explore;
pub mod how_it_works;
pub mod forms;
pub mod preview;

// Re-export Language and context functions for convenience
pub use language::{Language, provide_language_context, use_language, use_set_language};

// Legacy Translations struct for backward compatibility
pub struct Translations;

impl Translations {
    // Header
    pub fn connect_wallet(lang: Language) -> &'static str {
        header::connect_wallet(lang)
    }

    pub fn toggle_theme(lang: Language) -> &'static str {
        header::toggle_theme(lang)
    }

    // Hero section
    pub fn title(lang: Language) -> &'static str {
        common::title(lang)
    }

    pub fn subtitle(lang: Language) -> &'static str {
        common::subtitle(lang)
    }

    // Upload section
    pub fn drag_csv_here(lang: Language) -> &'static str {
        common::drag_csv_here(lang)
    }

    pub fn or_click_to_select(lang: Language) -> &'static str {
        common::or_click_to_select(lang)
    }

    pub fn supported_formats(lang: Language) -> &'static str {
        common::supported_formats(lang)
    }

    pub fn auto_transform_ai(lang: Language) -> &'static str {
        common::auto_transform_ai(lang)
    }

    pub fn choose_csv_file(lang: Language) -> &'static str {
        common::choose_csv_file(lang)
    }

    // Logs panel
    pub fn logs(lang: Language) -> &'static str {
        common::logs(lang)
    }

    pub fn clear(lang: Language) -> &'static str {
        common::clear(lang)
    }
}

/// Translate a key using the current language context
pub fn t(key: &str) -> String {
    let lang = use_language().get();
    translate(key, lang)
}

/// Translate a key for a specific language
pub fn translate(key: &str, lang: Language) -> String {
    // Try each module's translate function in order
    if let Some(translation) = header::translate(key, lang) {
        return translation.to_string();
    }
    if let Some(translation) = common::translate(key, lang) {
        return translation.to_string();
    }
    if let Some(translation) = nav::translate(key, lang) {
        return translation.to_string();
    }
    if let Some(translation) = home::translate(key, lang) {
        return translation.to_string();
    }
    if let Some(translation) = register::translate(key, lang) {
        return translation.to_string();
    }
    if let Some(translation) = protect::translate(key, lang) {
        return translation.to_string();
    }
    if let Some(translation) = explore::translate(key, lang) {
        return translation.to_string();
    }
    if let Some(translation) = how_it_works::translate(key, lang) {
        return translation.to_string();
    }
    if let Some(translation) = forms::translate(key, lang) {
        return translation.to_string();
    }
    if let Some(translation) = preview::translate(key, lang) {
        return translation.to_string();
    }
    
    // Fallback: return the key itself
    key.to_string()
}

