//! Theme state management for dark/light mode.
//!
//! Provides a global context for theme switching with automatic DOM updates.

use leptos::*;

/// Theme variants
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    /// Toggle between Dark and Light
    pub fn toggle(self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }

    /// Get the theme attribute value for DOM
    pub fn to_attribute(&self) -> &'static str {
        match self {
            Theme::Dark => "", // Empty string for dark (default)
            Theme::Light => "light",
        }
    }

    /// Get the theme name as string
    pub fn name(&self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
        }
    }
}

/// Theme state context
#[derive(Clone, Copy)]
pub struct ThemeState {
    theme: RwSignal<Theme>,
}

impl ThemeState {
    /// Create a new theme state with default (Dark) theme
    pub fn new() -> Self {
        Self {
            theme: create_rw_signal(Theme::default()),
        }
    }

    /// Create a new theme state with initial theme
    pub fn with_initial(initial: Theme) -> Self {
        Self {
            theme: create_rw_signal(initial),
        }
    }

    /// Get current theme
    pub fn get(&self) -> Theme {
        self.theme.get()
    }

    /// Set theme and update DOM
    pub fn set(&self, theme: Theme) {
        self.theme.set(theme);
        Self::update_dom_theme(theme);
    }

    /// Toggle theme and update DOM
    pub fn toggle(&self) {
        self.theme.update(|t| {
            *t = t.toggle();
            Self::update_dom_theme(*t);
        });
    }

    /// Get the reactive signal (for use in views)
    pub fn signal(&self) -> RwSignal<Theme> {
        self.theme
    }

    /// Update the DOM with the current theme
    fn update_dom_theme(theme: Theme) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute("data-theme", theme.to_attribute());
                }
            }
        }
    }

    /// Initialize theme from system preference (prefers-color-scheme)
    /// Note: Simplified version, always defaults to Dark
    /// For full system preference detection, enable web-sys features in Cargo.toml
    pub fn from_system() -> Self {
        // TODO: Implement system preference detection with proper web-sys features
        let theme = Theme::Dark;
        let state = Self::with_initial(theme);
        Self::update_dom_theme(theme);
        state
    }

    /// Initialize theme from localStorage (if available)
    pub fn from_storage() -> Self {
        let theme = if let Some(window) = web_sys::window() {
            window
                .local_storage()
                .ok()
                .flatten()
                .and_then(|storage| storage.get_item("theme").ok().flatten())
                .and_then(|theme_str| match theme_str.as_str() {
                    "light" => Some(Theme::Light),
                    "dark" => Some(Theme::Dark),
                    _ => None,
                })
                .unwrap_or_else(|| {
                    // Fallback to system preference
                    Self::from_system().get()
                })
        } else {
            Theme::Dark
        };

        let state = Self::with_initial(theme);
        Self::update_dom_theme(theme);
        state
    }

    /// Save theme to localStorage
    pub fn save_to_storage(&self) {
        let theme = self.get();
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme", theme.to_attribute());
            }
        }
    }

    /// Toggle theme and save to localStorage
    pub fn toggle_and_save(&self) {
        self.toggle();
        self.save_to_storage();
    }
}

impl Default for ThemeState {
    fn default() -> Self {
        Self::new()
    }
}

// Context providers
#[derive(Clone, Copy)]
struct ThemeContext(ThemeState);

/// Provide theme context to the component tree
pub fn provide_theme_context() -> ThemeState {
    let state = ThemeState::from_storage();
    provide_context(ThemeContext(state));
    state
}

/// Provide theme context with a specific initial theme
pub fn provide_theme_context_with(theme: Theme) -> ThemeState {
    let state = ThemeState::with_initial(theme);
    provide_context(ThemeContext(state));
    state
}

/// Use theme state from context
pub fn use_theme() -> ThemeState {
    expect_context::<ThemeContext>().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_toggle() {
        let dark = Theme::Dark;
        let light = Theme::Light;

        assert_eq!(dark.toggle(), light);
        assert_eq!(light.toggle(), dark);
    }

    #[test]
    fn test_theme_attribute() {
        assert_eq!(Theme::Dark.to_attribute(), "");
        assert_eq!(Theme::Light.to_attribute(), "light");
    }

    #[test]
    fn test_theme_name() {
        assert_eq!(Theme::Dark.name(), "Dark");
        assert_eq!(Theme::Light.name(), "Light");
    }
}

