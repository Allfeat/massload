//! Notification/Toast state management.
//!
//! Provides a global context for displaying toast notifications and alerts.

use leptos::*;
use std::time::Duration;

/// Notification severity level
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationLevel {
    /// Get the CSS class for this level
    pub fn css_class(&self) -> &'static str {
        match self {
            NotificationLevel::Info => "notification-info",
            NotificationLevel::Success => "notification-success",
            NotificationLevel::Warning => "notification-warning",
            NotificationLevel::Error => "notification-error",
        }
    }

    /// Get the emoji/icon for this level
    pub fn icon(&self) -> &'static str {
        match self {
            NotificationLevel::Info => "ℹ️",
            NotificationLevel::Success => "✅",
            NotificationLevel::Warning => "⚠️",
            NotificationLevel::Error => "❌",
        }
    }
}

/// A single notification
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notification {
    pub id: usize,
    pub level: NotificationLevel,
    pub title: Option<String>,
    pub message: String,
    pub duration: Option<Duration>,
}

impl Notification {
    /// Create a new notification
    pub fn new(level: NotificationLevel, message: impl Into<String>) -> Self {
        Self {
            id: 0, // Will be assigned by NotificationState
            level,
            title: None,
            message: message.into(),
            duration: Some(Duration::from_secs(5)),
        }
    }

    /// Set the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Make the notification persistent (no auto-dismiss)
    pub fn persistent(mut self) -> Self {
        self.duration = None;
        self
    }

    // Convenience constructors
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(NotificationLevel::Info, message)
    }

    pub fn success(message: impl Into<String>) -> Self {
        Self::new(NotificationLevel::Success, message)
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(NotificationLevel::Warning, message)
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(NotificationLevel::Error, message)
    }
}

/// Notification state context
#[derive(Clone, Copy)]
pub struct NotificationState {
    notifications: RwSignal<Vec<Notification>>,
    next_id: RwSignal<usize>,
}

impl NotificationState {
    /// Create a new notification state
    pub fn new() -> Self {
        Self {
            notifications: create_rw_signal(Vec::new()),
            next_id: create_rw_signal(0),
        }
    }

    /// Add a notification
    pub fn add(&self, mut notification: Notification) {
        let id = self.next_id.get();
        notification.id = id;
        self.next_id.set(id + 1);

        self.notifications.update(|notifs| {
            notifs.push(notification.clone());
        });

        // Auto-dismiss if duration is set
        if let Some(duration) = notification.duration {
            let notif_id = notification.id;
            let state = *self;
            
            set_timeout(
                move || {
                    state.dismiss(notif_id);
                },
                duration,
            );
        }
    }

    /// Dismiss a notification by ID
    pub fn dismiss(&self, id: usize) {
        self.notifications.update(|notifs| {
            notifs.retain(|n| n.id != id);
        });
    }

    /// Dismiss all notifications
    pub fn dismiss_all(&self) {
        self.notifications.set(Vec::new());
    }

    /// Get all notifications (reactive)
    pub fn get_all(&self) -> Vec<Notification> {
        self.notifications.get()
    }

    /// Get the signal for rendering
    pub fn signal(&self) -> RwSignal<Vec<Notification>> {
        self.notifications
    }

    // Convenience methods for common notification types
    pub fn info(&self, message: impl Into<String>) {
        self.add(Notification::info(message));
    }

    pub fn success(&self, message: impl Into<String>) {
        self.add(Notification::success(message));
    }

    pub fn warning(&self, message: impl Into<String>) {
        self.add(Notification::warning(message));
    }

    pub fn error(&self, message: impl Into<String>) {
        self.add(Notification::error(message));
    }
}

impl Default for NotificationState {
    fn default() -> Self {
        Self::new()
    }
}

// Context providers
#[derive(Clone, Copy)]
struct NotificationContext(NotificationState);

/// Provide notification context to the component tree
pub fn provide_notification_context() -> NotificationState {
    let state = NotificationState::new();
    provide_context(NotificationContext(state));
    state
}

/// Use notification state from context
pub fn use_notifications() -> NotificationState {
    expect_context::<NotificationContext>().0
}

/// A component to render notifications
#[component]
pub fn NotificationContainer() -> impl IntoView {
    let state = use_notifications();
    let notifications = state.signal();

    view! {
        <div class="notification-container">
            <For
                each=move || notifications.get()
                key=|n| n.id
                children=move |notification| {
                    let id = notification.id;
                    let on_close = move |_| {
                        state.dismiss(id);
                    };

                    view! {
                        <div class=format!("notification {}", notification.level.css_class())>
                            <div class="notification-icon">{notification.level.icon()}</div>
                            <div class="notification-content">
                                {notification.title.as_ref().map(|title| {
                                    view! {
                                        <div class="notification-title">{title.clone()}</div>
                                    }
                                })}
                                <div class="notification-message">{notification.message.clone()}</div>
                            </div>
                            <button class="notification-close" on:click=on_close>
                                "×"
                            </button>
                        </div>
                    }
                }
            />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_level_css() {
        assert_eq!(NotificationLevel::Info.css_class(), "notification-info");
        assert_eq!(NotificationLevel::Success.css_class(), "notification-success");
        assert_eq!(NotificationLevel::Warning.css_class(), "notification-warning");
        assert_eq!(NotificationLevel::Error.css_class(), "notification-error");
    }

    #[test]
    fn test_notification_creation() {
        let notif = Notification::info("Test message");
        assert_eq!(notif.level, NotificationLevel::Info);
        assert_eq!(notif.message, "Test message");
        assert!(notif.title.is_none());
        assert!(notif.duration.is_some());
    }

    #[test]
    fn test_notification_with_title() {
        let notif = Notification::success("Success!").with_title("Great");
        assert_eq!(notif.title, Some("Great".to_string()));
    }

    #[test]
    fn test_notification_persistent() {
        let notif = Notification::error("Error!").persistent();
        assert!(notif.duration.is_none());
    }
}

