//! Sidebar navigation component - Tanssi-style left menu
//! Based on https://apps.tanssi.network/
//!
//! This is a generic sidebar that can be configured with custom navigation items.

use leptos::*;
use leptos_router::*;

/// A navigation item in the sidebar
#[derive(Clone, Debug)]
pub struct NavItem {
    /// The path to navigate to
    pub path: String,
    /// The label to display
    pub label: String,
    /// The icon component (as a view)
    pub icon: View,
    /// Whether this is an external link
    pub is_external: bool,
}

impl NavItem {
    /// Create a new internal navigation item
    pub fn new(path: impl Into<String>, label: impl Into<String>, icon: View) -> Self {
        Self {
            path: path.into(),
            label: label.into(),
            icon,
            is_external: false,
        }
    }

    /// Create a new external navigation item
    pub fn external(path: impl Into<String>, label: impl Into<String>, icon: View) -> Self {
        Self {
            path: path.into(),
            label: label.into(),
            icon,
            is_external: true,
        }
    }
}

/// A section in the sidebar (group of nav items)
#[derive(Clone, Debug)]
pub struct NavSection {
    /// Navigation items in this section
    pub items: Vec<NavItem>,
}

impl NavSection {
    /// Create a new navigation section
    pub fn new(items: Vec<NavItem>) -> Self {
        Self { items }
    }
}

/// Sidebar navigation with Tanssi-style menu
#[component]
pub fn Sidebar(
    /// Navigation sections to display
    sections: Vec<NavSection>,
    /// Optional footer content
    #[prop(optional)]
    footer: Option<View>,
) -> impl IntoView {
    let location = use_location();
    
    // Check if a path is active
    let is_active = move |path: &str| {
        let current = location.pathname.get();
        if path == "/" {
            current == "/"
        } else {
            current.starts_with(path)
        }
    };

    view! {
        <aside class="sidebar">
            <nav class="sidebar-nav">
                {sections.into_iter().enumerate().map(|(idx, section)| {
                    view! {
                        <>
                        {if idx > 0 {
                            view! { <div class="nav-divider"></div> }.into_view()
                        } else {
                            view! { <></> }.into_view()
                        }}
                        <div class="nav-section">
                            {section.items.into_iter().map(|item| {
                                let path = item.path.clone();
                                let path_for_active = path.clone();
                                
                                if item.is_external {
                                    view! {
                                        <a 
                                            href=path
                                            target="_blank"
                                            class="nav-item external"
                                        >
                                            <span class="nav-icon">{item.icon}</span>
                                            <span class="nav-label">{item.label}</span>
                                            <span class="nav-external">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                    <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
                                                    <polyline points="15 3 21 3 21 9"/>
                                                    <line x1="10" x2="21" y1="14" y2="3"/>
                                                </svg>
                                            </span>
                                        </a>
                                    }.into_view()
                                } else {
                                    view! {
                                        <A 
                                            href=path
                                            class=move || if is_active(&path_for_active) { 
                                                "nav-item active" 
                                            } else { 
                                                "nav-item" 
                                            }
                                        >
                                            <span class="nav-icon">{item.icon}</span>
                                            <span class="nav-label">{item.label}</span>
                                        </A>
                                    }.into_view()
                                }
                            }).collect_view()}
                        </div>
                        </>
                    }
                }).collect_view()}
            </nav>
            
            {move || {
                if let Some(ref footer_view) = footer {
                    view! {
                        <div class="sidebar-footer">
                            {footer_view.clone()}
                        </div>
                    }.into_view()
                } else {
                    view! { <></> }.into_view()
                }
            }}
        </aside>
    }
}

/// A simple network status footer component
#[component]
pub fn NetworkStatus(
    /// Network name to display
    network_name: String,
    /// Whether the network is online
    #[prop(default = true)]
    online: bool,
) -> impl IntoView {
    view! {
        <div class="network-status">
            <span class="status-dot" class:online=online class:offline=!online></span>
            <span class="status-text">{network_name}</span>
        </div>
    }
}

