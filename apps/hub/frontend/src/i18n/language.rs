//! Language definitions and metadata.
//! Supports: English, French, Spanish, German, Japanese, Korean, Greek

use leptos::*;

/// Supported languages
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    English,
    French,
    Spanish,
    German,
    Japanese,
    Korean,
    Greek,
}

impl Language {
    /// Get the flag emoji for this language
    pub fn flag(&self) -> &'static str {
        match self {
            Language::English => "🇬🇧",
            Language::French => "🇫🇷",
            Language::Spanish => "🇪🇸",
            Language::German => "🇩🇪",
            Language::Japanese => "🇯🇵",
            Language::Korean => "🇰🇷",
            Language::Greek => "🇬🇷",
        }
    }

    /// Get the native name of this language
    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::French => "Français",
            Language::Spanish => "Español",
            Language::German => "Deutsch",
            Language::Japanese => "日本語",
            Language::Korean => "한국어",
            Language::Greek => "Ελληνικά",
        }
    }

    /// Get all available languages
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::French,
            Language::Spanish,
            Language::German,
            Language::Japanese,
            Language::Korean,
            Language::Greek,
        ]
    }
}

// Context providers for reactive language switching
#[derive(Clone, Copy)]
struct LanguageContext(RwSignal<Language>);

/// Provide language context to the component tree
pub fn provide_language_context() -> RwSignal<Language> {
    let lang = create_rw_signal(Language::default());
    provide_context(LanguageContext(lang));
    lang
}

/// Use language from context (returns RwSignal for .get())
pub fn use_language() -> RwSignal<Language> {
    expect_context::<LanguageContext>().0
}

/// Use language setter from context
pub fn use_set_language() -> WriteSignal<Language> {
    expect_context::<LanguageContext>().0.write_only()
}

