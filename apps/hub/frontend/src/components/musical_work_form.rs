//! Musical Work registration form
//! Based on /schemas/midds-musical-work-grouped.json

use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use crate::i18n::t;
use crate::midds::*;

/// Form state for creators
#[derive(Clone, Debug)]
pub struct CreatorForm {
    pub ipi: String,
    pub isni: String,
    pub role: String,
}

/// Musical Work form component
#[component]
pub fn MusicalWorkForm(
    /// Callback when form is submitted successfully
    on_submit: impl Fn(MusicalWork) + 'static,
) -> impl IntoView {
    // Form fields
    let iswc = create_rw_signal(String::new());
    let title = create_rw_signal(String::new());
    let creation_year = create_rw_signal(String::new());
    let instrumental = create_rw_signal(false);
    let language = create_rw_signal(String::new());
    let bpm = create_rw_signal(String::new());
    let key = create_rw_signal(String::new());
    let work_type = create_rw_signal("Original".to_string());
    
    let opus = create_rw_signal(String::new());
    let catalog_number = create_rw_signal(String::new());
    let number_of_voices = create_rw_signal(String::new());
    
    let creators = create_rw_signal(Vec::<CreatorForm>::new());
    
    // Validation errors
    let validation_errors = create_rw_signal(Vec::<String>::new());
    
    // Add creator
    let add_creator = move |_| {
        creators.update(|cs| {
            cs.push(CreatorForm {
                ipi: String::new(),
                isni: String::new(),
                role: "Composer".to_string(),
            });
        });
    };
    
    // Remove creator
    let remove_creator = move |index: usize| {
        creators.update(|cs| {
            if cs.len() > index {
                cs.remove(index);
            }
        });
    };
    
    // Handle submit
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        validation_errors.set(vec![]);
        
        // Build creators
        let creator_list: Vec<Creator> = creators.get()
            .iter()
            .filter_map(|c| {
                // Must have either IPI or ISNI
                if !c.ipi.is_empty() {
                    if let Ok(ipi_num) = c.ipi.parse::<u32>() {
                        return Some(Creator {
                            id: PartyId::Ipi(ipi_num),
                            role: c.role.clone(),
                        });
                    }
                }
                if !c.isni.is_empty() {
                    return Some(Creator {
                        id: PartyId::Isni(c.isni.clone()),
                        role: c.role.clone(),
                    });
                }
                None
            })
            .collect();
        
        if creator_list.is_empty() {
            validation_errors.set(vec!["At least one valid creator is required".to_string()]);
            return;
        }
        
        // Build classical info
        let classical_info = if !opus.get().is_empty() 
            || !catalog_number.get().is_empty() 
            || !number_of_voices.get().is_empty() {
            Some(ClassicalInfo {
                opus: if opus.get().is_empty() { None } else { Some(opus.get()) },
                catalog_number: if catalog_number.get().is_empty() { None } else { Some(catalog_number.get()) },
                number_of_voices: number_of_voices.get().parse().ok(),
            })
        } else {
            None
        };
        
        // Build MusicalWork
        let work = MusicalWork {
            iswc: iswc.get().to_uppercase(),
            title: title.get(),
            creators: creator_list,
            creation_year: creation_year.get().parse().ok(),
            instrumental: if instrumental.get() { Some(true) } else { None },
            language: if language.get().is_empty() { None } else { Some(language.get()) },
            bpm: bpm.get().parse().ok(),
            key: if key.get().is_empty() { None } else { Some(key.get()) },
            work_type: match work_type.get().as_str() {
                "Original" => Some(WorkType::Original),
                "Medley" => Some(WorkType::Medley),
                "Mashup" => Some(WorkType::Mashup),
                "Adaptation" => Some(WorkType::Adaptation),
                _ => None,
            },
            classical_info,
            participants: vec![],
        };
        
        // Validate against schema
        match work.validate() {
            Ok(_) => {
                on_submit(work);
            }
            Err(errors) => {
                validation_errors.set(errors);
            }
        }
    };
    
    view! {
        // Single Card wrapper (like TS original)
        <div class="form-card-wrapper">
            <div class="form-card-header">
                <h2 class="form-card-title">{t("register.musical_work.title")}</h2>
            </div>
            
            <form class="musical-work-form" on:submit=handle_submit>
                // Validation errors (global)
                {move || {
                    let errors = validation_errors.get();
                    if !errors.is_empty() {
                        view! {
                            <div class="form-errors">
                                <h4>"Validation Errors:"</h4>
                                <ul>
                                    {errors.into_iter().map(|err| view! {
                                        <li>{err}</li>
                                    }).collect::<Vec<_>>()}
                                </ul>
                            </div>
                        }.into_view()
                    } else {
                        view! { <></> }.into_view()
                    }
                }}
                
                // Basic fields (2 columns: ISWC + Title)
                <div class="form-section">
                    <div class="form-row-2">
                <h3 class="form-section-title">{t("register.form.basic_info")}</h3>
                
                <div class="form-row">
                    // ISWC (required)
                    <div class="form-group">
                        <label class="form-label">
                            {t("register.form.iswc")} " *"
                        </label>
                        <input
                            type="text"
                            class="form-input"
                            placeholder="T1234567890"
                            maxlength="11"
                            value={move || iswc.get()}
                            on:input=move |ev| {
                                iswc.set(event_target_value(&ev).to_uppercase());
                            }
                        />
                        <div class="form-hint">"Format: T + 10 digits"</div>
                    </div>
                    
                    // Title (required)
                    <div class="form-group">
                        <label class="form-label">
                            {t("register.form.title")} " *"
                        </label>
                        <input
                            type="text"
                            class="form-input"
                            maxlength="256"
                            value={move || title.get()}
                            on:input=move |ev| {
                                title.set(event_target_value(&ev));
                            }
                        />
                    </div>
                </div>
                
                <div class="form-row">
                    // Creation Year (optional)
                    <div class="form-group">
                        <label class="form-label">{t("register.form.creation_year")}</label>
                        <input
                            type="number"
                            class="form-input"
                            placeholder="2024"
                            min="1000"
                            max="9999"
                            value={move || creation_year.get()}
                            on:input=move |ev| {
                                creation_year.set(event_target_value(&ev));
                            }
                        />
                    </div>
                    
                    // Work Type
                    <div class="form-group">
                        <label class="form-label">{t("register.form.work_type")}</label>
                        <select
                            class="form-select"
                            on:change=move |ev| {
                                work_type.set(event_target_value(&ev));
                            }
                        >
                            <option value="Original" selected>{t("register.form.work_type_original")}</option>
                            <option value="Medley">{t("register.form.work_type_medley")}</option>
                            <option value="Mashup">{t("register.form.work_type_mashup")}</option>
                            <option value="Adaptation">{t("register.form.work_type_adaptation")}</option>
                        </select>
                    </div>
                    
                    // Instrumental checkbox
                    <div class="form-group form-checkbox-group">
                        <input
                            type="checkbox"
                            class="form-checkbox"
                            checked={move || instrumental.get()}
                            on:change=move |ev| {
                                instrumental.set(event_target_checked(&ev));
                            }
                        />
                        <label class="form-checkbox-label">{t("register.form.instrumental")}</label>
                    </div>
                </div>
            </div>
            
            // Creators (required)
            <div class="form-card">
                <h3 class="form-section-title">{t("register.form.creators")} " *"</h3>
                
                {move || creators.get().iter().enumerate().map(|(index, creator)| {
                    let creator = creator.clone();
                    view! {
                        <div class="creator-row">
                            <div class="form-row">
                                <div class="form-group">
                                    <label class="form-label">"IPI (9 digits)"</label>
                                    <input
                                        type="text"
                                        class="form-input"
                                        placeholder="123456789"
                                        maxlength="9"
                                        value={creator.ipi.clone()}
                                        on:input=move |ev| {
                                            let value = event_target_value(&ev);
                                            creators.update(|cs| {
                                                if let Some(c) = cs.get_mut(index) {
                                                    c.ipi = value;
                                                }
                                            });
                                        }
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">"ISNI (16 digits)"</label>
                                    <input
                                        type="text"
                                        class="form-input"
                                        placeholder="1234567890123456"
                                        maxlength="16"
                                        value={creator.isni.clone()}
                                        on:input=move |ev| {
                                            let value = event_target_value(&ev);
                                            creators.update(|cs| {
                                                if let Some(c) = cs.get_mut(index) {
                                                    c.isni = value;
                                                }
                                            });
                                        }
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label class="form-label">{t("register.form.role")}</label>
                                    <select
                                        class="form-select"
                                        on:change=move |ev| {
                                            let value = event_target_value(&ev);
                                            creators.update(|cs| {
                                                if let Some(c) = cs.get_mut(index) {
                                                    c.role = value;
                                                }
                                            });
                                        }
                                    >
                                        <option value="Composer" selected={creator.role == "Composer"}>"Composer"</option>
                                        <option value="Author" selected={creator.role == "Author"}>"Author"</option>
                                        <option value="Arranger" selected={creator.role == "Arranger"}>"Arranger"</option>
                                        <option value="Adapter" selected={creator.role == "Adapter"}>"Adapter"</option>
                                        <option value="Publisher" selected={creator.role == "Publisher"}>"Publisher"</option>
                                    </select>
                                </div>
                                
                                <button
                                    type="button"
                                    class="btn btn-danger"
                                    on:click=move |_| remove_creator(index)
                                >
                                    "×"
                                </button>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
                
                <button type="button" class="btn btn-outline" on:click=add_creator>
                    "+ " {t("register.form.add_creator")}
                </button>
            </div>
            
            // Musical Attributes
            <div class="form-card">
                <h3 class="form-section-title">{t("register.form.musical_attributes")}</h3>
                
                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">{t("register.form.language")}</label>
                        <input
                            type="text"
                            class="form-input"
                            placeholder="English, French, Spanish..."
                            value={move || language.get()}
                            on:input=move |ev| {
                                language.set(event_target_value(&ev));
                            }
                        />
                    </div>
                    
                    <div class="form-group">
                        <label class="form-label">"BPM"</label>
                        <input
                            type="number"
                            class="form-input"
                            min="1"
                            max="65535"
                            value={move || bpm.get()}
                            on:input=move |ev| {
                                bpm.set(event_target_value(&ev));
                            }
                        />
                    </div>
                    
                    <div class="form-group">
                        <label class="form-label">{t("register.form.key")}</label>
                        <input
                            type="text"
                            class="form-input"
                            placeholder="C, Dm, F#..."
                            value={move || key.get()}
                            on:input=move |ev| {
                                key.set(event_target_value(&ev));
                            }
                        />
                    </div>
                </div>
            </div>
            
            // Classical Information (optional)
            <div class="form-card">
                <h3 class="form-section-title">{t("register.form.classical_info")}</h3>
                
                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">"Opus"</label>
                        <input
                            type="text"
                            class="form-input"
                            maxlength="128"
                            value={move || opus.get()}
                            on:input=move |ev| {
                                opus.set(event_target_value(&ev));
                            }
                        />
                    </div>
                    
                    <div class="form-group">
                        <label class="form-label">"Catalog Number"</label>
                        <input
                            type="text"
                            class="form-input"
                            maxlength="128"
                            value={move || catalog_number.get()}
                            on:input=move |ev| {
                                catalog_number.set(event_target_value(&ev));
                            }
                        />
                    </div>
                    
                    <div class="form-group">
                        <label class="form-label">"Number of Voices"</label>
                        <input
                            type="number"
                            class="form-input"
                            min="1"
                            value={move || number_of_voices.get()}
                            on:input=move |ev| {
                                number_of_voices.set(event_target_value(&ev));
                            }
                        />
                    </div>
                </div>
            </div>
            
            // Submit
            <div class="form-actions">
                <A href="/register" class="btn btn-outline">
                    {t("register.back")}
                </A>
                <button type="submit" class="btn btn-primary">
                    {t("register.form.submit")}
                </button>
            </div>
        </form>
    }
}

/// Helper to get checked state from input event
fn event_target_checked(ev: &web_sys::Event) -> bool {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.checked())
        .unwrap_or(false)
}
