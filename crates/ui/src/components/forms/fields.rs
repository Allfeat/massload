//! Reusable form field components.
//!
//! These components provide consistent styling and behavior for form inputs
//! across all Allfeat applications.

use leptos::*;

/// A styled text input component
#[component]
pub fn TextInput(
    /// Field label
    label: String,
    /// Field name/id
    name: String,
    /// Current value
    value: Signal<String>,
    /// Callback when value changes
    on_change: Callback<String>,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// Whether the field is required
    #[prop(default = false)]
    required: bool,
    /// Whether the field is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Help text to display below the field
    #[prop(optional)]
    help_text: Option<String>,
    /// Error message to display
    #[prop(optional)]
    error: Signal<Option<String>>,
) -> impl IntoView {
    let input_id = format!("input-{}", name);
    
    view! {
        <div class="form-field">
            <label class="form-label" for=input_id.clone()>
                {label.clone()}
                {if required { " *" } else { "" }}
            </label>
            <input
                type="text"
                id=input_id
                name=name.clone()
                class="form-input"
                class:error=move || error.get().is_some()
                value=move || value.get()
                placeholder=placeholder.unwrap_or_default()
                required=required
                disabled=disabled
                on:input=move |ev| {
                    on_change.call(event_target_value(&ev));
                }
            />
            {move || {
                if let Some(err) = error.get() {
                    view! { <span class="form-error">{err}</span> }.into_view()
                } else if let Some(ref help) = help_text {
                    view! { <span class="form-help">{help}</span> }.into_view()
                } else {
                    view! { <></> }.into_view()
                }
            }}
        </div>
    }
}

/// A styled textarea component
#[component]
pub fn TextArea(
    /// Field label
    label: String,
    /// Field name/id
    name: String,
    /// Current value
    value: Signal<String>,
    /// Callback when value changes
    on_change: Callback<String>,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<String>,
    /// Number of rows
    #[prop(default = 4)]
    rows: u32,
    /// Whether the field is required
    #[prop(default = false)]
    required: bool,
    /// Whether the field is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Help text to display below the field
    #[prop(optional)]
    help_text: Option<String>,
    /// Error message to display
    #[prop(optional)]
    error: Signal<Option<String>>,
) -> impl IntoView {
    let input_id = format!("textarea-{}", name);
    
    view! {
        <div class="form-field">
            <label class="form-label" for=input_id.clone()>
                {label.clone()}
                {if required { " *" } else { "" }}
            </label>
            <textarea
                id=input_id
                name=name.clone()
                class="form-textarea"
                class:error=move || error.get().is_some()
                rows=rows
                placeholder=placeholder.unwrap_or_default()
                required=required
                disabled=disabled
                on:input=move |ev| {
                    on_change.call(event_target_value(&ev));
                }
            >
                {move || value.get()}
            </textarea>
            {move || {
                if let Some(err) = error.get() {
                    view! { <span class="form-error">{err}</span> }.into_view()
                } else if let Some(ref help) = help_text {
                    view! { <span class="form-help">{help}</span> }.into_view()
                } else {
                    view! { <></> }.into_view()
                }
            }}
        </div>
    }
}

/// A styled select/dropdown component
#[component]
pub fn Select(
    /// Field label
    label: String,
    /// Field name/id
    name: String,
    /// Current value
    value: Signal<String>,
    /// Callback when value changes
    on_change: Callback<String>,
    /// Options as (value, label) pairs
    options: Vec<(String, String)>,
    /// Whether the field is required
    #[prop(default = false)]
    required: bool,
    /// Whether the field is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Help text to display below the field
    #[prop(optional)]
    help_text: Option<String>,
    /// Error message to display
    #[prop(optional)]
    error: Signal<Option<String>>,
) -> impl IntoView {
    let input_id = format!("select-{}", name);
    
    view! {
        <div class="form-field">
            <label class="form-label" for=input_id.clone()>
                {label.clone()}
                {if required { " *" } else { "" }}
            </label>
            <select
                id=input_id
                name=name.clone()
                class="form-select"
                class:error=move || error.get().is_some()
                required=required
                disabled=disabled
                on:change=move |ev| {
                    on_change.call(event_target_value(&ev));
                }
            >
                <option value="" disabled=true selected=move || value.get().is_empty()>
                    "Select an option..."
                </option>
                {options.iter().map(|(val, label)| {
                    let val_clone = val.clone();
                    view! {
                        <option 
                            value=val.clone()
                            selected=move || value.get() == val_clone
                        >
                            {label}
                        </option>
                    }
                }).collect_view()}
            </select>
            {move || {
                if let Some(err) = error.get() {
                    view! { <span class="form-error">{err}</span> }.into_view()
                } else if let Some(ref help) = help_text {
                    view! { <span class="form-help">{help}</span> }.into_view()
                } else {
                    view! { <></> }.into_view()
                }
            }}
        </div>
    }
}

/// A styled checkbox component
#[component]
pub fn Checkbox(
    /// Field label
    label: String,
    /// Field name/id
    name: String,
    /// Current checked state
    checked: Signal<bool>,
    /// Callback when checked state changes
    on_change: Callback<bool>,
    /// Whether the field is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Help text to display below the field
    #[prop(optional)]
    help_text: Option<String>,
) -> impl IntoView {
    let input_id = format!("checkbox-{}", name);
    
    view! {
        <div class="form-field form-checkbox">
            <input
                type="checkbox"
                id=input_id.clone()
                name=name.clone()
                class="form-checkbox-input"
                checked=move || checked.get()
                disabled=disabled
                on:change=move |ev| {
                    on_change.call(event_target_checked(&ev));
                }
            />
            <label class="form-checkbox-label" for=input_id>
                {label}
            </label>
            {move || {
                if let Some(ref help) = help_text {
                    view! { <span class="form-help">{help}</span> }.into_view()
                } else {
                    view! { <></> }.into_view()
                }
            }}
        </div>
    }
}

/// A styled button component
#[component]
pub fn Button(
    /// Button text/children
    children: Children,
    /// Button type (button, submit, reset)
    #[prop(default = "button".to_string())]
    button_type: String,
    /// Button variant (primary, secondary, danger)
    #[prop(default = "primary".to_string())]
    variant: String,
    /// Whether the button is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Whether the button is in loading state
    #[prop(default = false)]
    loading: bool,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let class_name = format!("btn btn-{}", variant);
    
    view! {
        <button
            type=button_type
            class=class_name
            class:loading=loading
            disabled=disabled || loading
            on:click=move |ev| {
                if let Some(handler) = on_click {
                    handler.call(ev);
                }
            }
        >
            {if loading {
                view! { <span class="btn-spinner"></span> }.into_view()
            } else {
                view! { <></> }.into_view()
            }}
            {children()}
        </button>
    }
}

