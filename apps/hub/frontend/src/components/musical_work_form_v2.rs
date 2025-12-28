use leptos::*;
use leptos_router::A;
use crate::i18n::t;
use crate::midds::*;
use crate::services::blockchain::BlockchainService;

// Temporary form state for adding creators
#[derive(Clone, Debug, Default)]
struct CreatorFormData {
    ipi: String,
    isni: String,
    selected_roles: Vec<String>,
}

// Temporary form state for adding publishers
#[derive(Clone, Debug, Default)]
struct PublisherFormData {
    ipi: String,
    isni: String,
}

// Available creator roles
const CREATOR_ROLES: &[&str] = &["Composer", "Lyricist", "Arranger", "Adapter"];

// Grouped creator for display (one IPI/ISNI can have multiple roles)
#[derive(Clone, Debug)]
struct GroupedCreator {
    party_id: PartyId,
    roles: Vec<String>,
}

#[component]
pub fn MusicalWorkFormV2() -> impl IntoView {
    // Form fields
    let (iswc, set_iswc) = create_signal(String::new());
    let (title, set_title) = create_signal(String::new());
    let (creation_year, set_creation_year) = create_signal(String::new());
    let (work_type, set_work_type) = create_signal("Original".to_string());
    let (instrumental, set_instrumental) = create_signal(false);
    let (language, set_language) = create_signal(String::new());
    let (bpm, set_bpm) = create_signal(String::new());
    let (key, set_key) = create_signal(String::new());
    
    // Classical info
    let (opus, set_opus) = create_signal(String::new());
    let (catalog_number, set_catalog_number) = create_signal(String::new());
    let (number_of_voices, set_number_of_voices) = create_signal(String::new());
    
    // Creators (flat list, one per role)
    let (creators, set_creators) = create_signal::<Vec<Creator>>(vec![]);
    let (creator_form, set_creator_form) = create_signal(CreatorFormData::default());
    
    // Publishers (participants)
    let (publishers, set_publishers) = create_signal::<Vec<Creator>>(vec![]);
    let (publisher_form, set_publisher_form) = create_signal(PublisherFormData::default());
    
    // Validation & submission
    let (validation_errors, set_validation_errors) = create_signal::<Vec<String>>(vec![]);
    let (is_submitting, set_is_submitting) = create_signal(false);
    let (submission_success, set_submission_success) = create_signal(false);
    
    // Wallet state from context (set in main.rs)
    let wallet_address = use_context::<ReadSignal<Option<String>>>()
        .unwrap_or_else(|| create_signal(None).0);
    let wallet_connected = use_context::<ReadSignal<bool>>()
        .unwrap_or_else(|| create_signal(false).0);
    
    // Group creators by party ID for display
    let grouped_creators = move || {
        let all_creators = creators.get();
        let mut groups: Vec<GroupedCreator> = vec![];
        let mut processed: Vec<usize> = vec![];
        
        for (i, creator) in all_creators.iter().enumerate() {
            if processed.contains(&i) {
                continue;
            }
            
            let mut group = GroupedCreator {
                party_id: creator.id.clone(),
                roles: vec![creator.role.clone()],
            };
            
            // Find all other creators with the same party ID
            for (j, other) in all_creators.iter().enumerate() {
                if j > i && !processed.contains(&j) && parties_match(&creator.id, &other.id) {
                    group.roles.push(other.role.clone());
                    processed.push(j);
                }
            }
            
            processed.push(i);
            groups.push(group);
        }
        
        groups
    };
    
    // Toggle role selection for creator form
    let toggle_role = move |role: String| {
        set_creator_form.update(|form| {
            if form.selected_roles.contains(&role) {
                form.selected_roles.retain(|r| r != &role);
            } else {
                form.selected_roles.push(role);
            }
        });
    };
    
    // Add creator(s) - one per selected role
    let add_creators = move |_| {
        let form = creator_form.get();
        
        // Validate
        if form.ipi.is_empty() && form.isni.is_empty() {
            return;
        }
        if form.selected_roles.is_empty() {
            return;
        }
        
        // Build PartyId
        let party_id = match (!form.ipi.is_empty(), !form.isni.is_empty()) {
            (true, false) => {
                if let Ok(ipi_num) = form.ipi.parse::<u32>() {
                    Some(PartyId::Ipi(ipi_num))
                } else {
                    None
                }
            },
            (false, true) => Some(PartyId::Isni(form.isni.clone())),
            (true, true) => {
                // For now, prioritize IPI if both are provided
                if let Ok(ipi_num) = form.ipi.parse::<u32>() {
                    Some(PartyId::Ipi(ipi_num))
                } else {
                    Some(PartyId::Isni(form.isni.clone()))
                }
            },
            _ => None,
        };
        
        if let Some(pid) = party_id {
            // Create one Creator per selected role
            let new_creators: Vec<Creator> = form.selected_roles.iter()
                .map(|role| Creator::new(pid.clone(), role.clone()))
                .collect();
            
            set_creators.update(|cs| {
                cs.extend(new_creators);
            });
            
            // Reset form
            set_creator_form.set(CreatorFormData::default());
        }
    };
    
    // Remove all creators with a given party ID
    let remove_creator_group = move |party_id: PartyId| {
        set_creators.update(|cs| {
            cs.retain(|c| !parties_match(&c.id, &party_id));
        });
    };
    
    // Add publisher
    let add_publisher = move |_| {
        let form = publisher_form.get();
        
        if form.ipi.is_empty() && form.isni.is_empty() {
            return;
        }
        
        let party_id = match (!form.ipi.is_empty(), !form.isni.is_empty()) {
            (true, false) => {
                if let Ok(ipi_num) = form.ipi.parse::<u32>() {
                    Some(PartyId::Ipi(ipi_num))
                } else {
                    None
                }
            },
            (false, true) => Some(PartyId::Isni(form.isni.clone())),
            (true, true) => {
                if let Ok(ipi_num) = form.ipi.parse::<u32>() {
                    Some(PartyId::Ipi(ipi_num))
                } else {
                    Some(PartyId::Isni(form.isni.clone()))
                }
            },
            _ => None,
        };
        
        if let Some(pid) = party_id {
            let publisher = Creator::new(pid, "Publisher");
            
            set_publishers.update(|ps| {
                ps.push(publisher);
            });
            
            set_publisher_form.set(PublisherFormData::default());
        }
    };
    
    // Remove publisher
    let remove_publisher = move |index: usize| {
        set_publishers.update(|ps| {
            if ps.len() > index {
                ps.remove(index);
            }
        });
    };
    
    // Handle submit
    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_validation_errors.set(vec![]);
        set_submission_success.set(false);
        
        // Build the musical work
        let mut all_participants = creators.get();
        all_participants.extend(publishers.get());
        
        let work = MusicalWork {
            iswc: iswc.get(),
            title: title.get(),
            creators: all_participants,
            // Note: alternative_titles and genre are backend-only fields
            // They are not available in WASM builds due to #[cfg(not(target_arch = "wasm32"))]
            creation_year: if !creation_year.get().is_empty() {
                creation_year.get().parse().ok()
            } else {
                None
            },
            instrumental: Some(instrumental.get()),
            language: if !language.get().is_empty() {
                Some(language.get())
            } else {
                None
            },
            bpm: if !bpm.get().is_empty() {
                bpm.get().parse().ok()
            } else {
                None
            },
            key: if !key.get().is_empty() {
                Some(key.get())
            } else {
                None
            },
            work_type: Some(match work_type.get().as_str() {
                "Medley" => WorkType::Medley,
                "Mashup" => WorkType::Mashup,
                "Adaptation" => WorkType::Adaptation,
                _ => WorkType::Original,
            }),
            classical_info: if !opus.get().is_empty() || !catalog_number.get().is_empty() || !number_of_voices.get().is_empty() {
                Some(ClassicalInfo {
                    opus: if !opus.get().is_empty() { Some(opus.get()) } else { None },
                    catalog_number: if !catalog_number.get().is_empty() { Some(catalog_number.get()) } else { None },
                    number_of_voices: if !number_of_voices.get().is_empty() {
                        number_of_voices.get().parse().ok()
                    } else {
                        None
                    },
                })
            } else {
                None
            },
            participants: vec![],
        };
        
        // Frontend validation
        match work.validate() {
            Ok(_) => {
                // Submit to blockchain
                let work_json = serde_json::to_value(&work).unwrap();
                
                // Debug: log the serialized JSON
                logging::log!("📦 Serialized work JSON:");
                logging::log!("{}", serde_json::to_string_pretty(&work_json).unwrap_or_default());
                
                set_is_submitting.set(true);
                
                spawn_local(async move {
                    let service = BlockchainService::new();
                    
                    match service.submit_single_work(work_json, wallet_address.get()).await {
                        Ok(result) => {
                            if result.success {
                                logging::log!("✅ Work submitted! Tx: {:?}", result.tx_hash);
                                set_submission_success.set(true);
                            } else {
                                let err_msg = result.error.unwrap_or_else(|| "Unknown error".to_string());
                                logging::error!("❌ Submission failed: {}", err_msg);
                                set_validation_errors.set(vec![err_msg]);
                            }
                        },
                        Err(e) => {
                            logging::error!("❌ Submission error: {}", e);
                            set_validation_errors.set(vec![e]);
                        }
                    }
                    
                    set_is_submitting.set(false);
                });
            },
            Err(errors) => {
                set_validation_errors.set(errors);
            }
        }
    };
    
    view! {
        <div class="register-form-page">
            <div class="form-breadcrumb">
                <A href="/register" class="breadcrumb-link">{t("register.back")}</A>
                " / "
                <span class="breadcrumb-current">{t("register.musical_work.title")}</span>
            </div>
            
            // Single Card wrapper (TypeScript style)
            <div class="ts-form-card">
                <div class="ts-form-header">
                    <h2 class="ts-form-title">{t("register.musical_work.title")}</h2>
                    <p class="ts-form-subtitle">"Register a musical work on the Allfeat blockchain"</p>
                </div>
                
                <form class="ts-form-body" on:submit=handle_submit>
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
                    
                    // Success message
                    {move || {
                        if submission_success.get() {
                            view! {
                                <div class="form-success">
                                    "✅ Musical work registered successfully!"
                                </div>
                            }.into_view()
                        } else {
                            view! { <></> }.into_view()
                        }
                    }}
                    
                    // ISWC + Title (2 columns)
                    <div class="grid-cols-2">
                        <div class="form-field">
                            <label for="iswc" class="form-label">"ISWC *"</label>
                            <input
                                type="text"
                                id="iswc"
                                class="form-input"
                                placeholder="T1234567890"
                                value=move || iswc.get()
                                on:input=move |ev| set_iswc.set(event_target_value(&ev))
                            />
                            <div class="form-hint">"Format: T followed by 10 digits"</div>
                        </div>
                        
                        <div class="form-field">
                            <label for="title" class="form-label">"Title *"</label>
                            <input
                                type="text"
                                id="title"
                                class="form-input"
                                placeholder="Song title"
                                value=move || title.get()
                                on:input=move |ev| set_title.set(event_target_value(&ev))
                            />
                        </div>
                    </div>
                    
                    // Year + WorkType + Instrumental (3 columns)
                    <div class="grid-cols-3">
                        <div class="form-field">
                            <label for="creation_year" class="form-label">"Creation Year"</label>
                            <input
                                type="number"
                                id="creation_year"
                                class="form-input"
                                placeholder="2024"
                                min="1000"
                                max="9999"
                                value=move || creation_year.get()
                                on:input=move |ev| set_creation_year.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <div class="form-field">
                            <label for="work_type" class="form-label">"Work Type"</label>
                            <select
                                id="work_type"
                                class="form-select"
                                on:change=move |ev| set_work_type.set(event_target_value(&ev))
                            >
                                <option value="Original" selected>"Original"</option>
                                <option value="Medley">"Medley"</option>
                                <option value="Mashup">"Mashup"</option>
                                <option value="Adaptation">"Adaptation"</option>
                            </select>
                        </div>
                        
                        <div class="form-field">
                            <label class="form-label checkbox-label">
                                <input
                                    type="checkbox"
                                    class="form-checkbox"
                                    checked=move || instrumental.get()
                                    on:change=move |ev| set_instrumental.set(event_target_checked(&ev))
                                />
                                " Instrumental"
                            </label>
                        </div>
                    </div>
                    
                    // Language + BPM + Key (3 columns)
                    <div class="grid-cols-3">
                        <div class="form-field">
                            <label for="language" class="form-label">"Language"</label>
                            <select
                                id="language"
                                class="form-select"
                                on:change=move |ev| set_language.set(event_target_value(&ev))
                            >
                                <option value="" selected>"Select language"</option>
                                <option value="English">"English"</option>
                                <option value="French">"French"</option>
                                <option value="Spanish">"Spanish"</option>
                                <option value="German">"German"</option>
                                <option value="Italian">"Italian"</option>
                                <option value="Portuguese">"Portuguese"</option>
                                <option value="Japanese">"Japanese"</option>
                                <option value="Korean">"Korean"</option>
                                <option value="Chinese">"Chinese"</option>
                                <option value="Arabic">"Arabic"</option>
                                <option value="Other">"Other"</option>
                            </select>
                        </div>
                        
                        <div class="form-field">
                            <label for="bpm" class="form-label">"BPM"</label>
                            <input
                                type="number"
                                id="bpm"
                                class="form-input"
                                placeholder="120"
                                min="1"
                                max="999"
                                value=move || bpm.get()
                                on:input=move |ev| set_bpm.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <div class="form-field">
                            <label for="key" class="form-label">"Key"</label>
                            <select
                                id="key"
                                class="form-select"
                                on:change=move |ev| set_key.set(event_target_value(&ev))
                            >
                                <option value="" selected>"Select key"</option>
                                <option value="C">"C"</option>
                                <option value="C#">"C#"</option>
                                <option value="D">"D"</option>
                                <option value="D#">"D#"</option>
                                <option value="E">"E"</option>
                                <option value="F">"F"</option>
                                <option value="F#">"F#"</option>
                                <option value="G">"G"</option>
                                <option value="G#">"G#"</option>
                                <option value="A">"A"</option>
                                <option value="A#">"A#"</option>
                                <option value="B">"B"</option>
                                <option value="Am">"Am"</option>
                                <option value="A#m">"A#m"</option>
                                <option value="Bm">"Bm"</option>
                                <option value="Cm">"Cm"</option>
                                <option value="C#m">"C#m"</option>
                                <option value="Dm">"Dm"</option>
                                <option value="D#m">"D#m"</option>
                                <option value="Em">"Em"</option>
                                <option value="Fm">"Fm"</option>
                                <option value="F#m">"F#m"</option>
                                <option value="Gm">"Gm"</option>
                                <option value="G#m">"G#m"</option>
                            </select>
                        </div>
                    </div>
                    
                    // CREATORS SECTION (Card style)
                    <div class="ts-section-card">
                        <h3 class="ts-section-title">"Creators *"</h3>
                        <p class="ts-section-subtitle">"Add creators with their roles (Composer, Lyricist, etc.)"</p>
                        
                        // Creator form (IPI + ISNI)
                        <div class="grid-cols-2">
                            <div class="form-field">
                                <label for="creator_ipi" class="form-label">"IPI code"</label>
                                <input
                                    type="text"
                                    id="creator_ipi"
                                    class="form-input"
                                    placeholder="123456789"
                                    inputmode="numeric"
                                    maxlength="11"
                                    value=move || creator_form.get().ipi
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev).chars()
                                            .filter(|c| c.is_numeric())
                                            .take(11)
                                            .collect::<String>();
                                        set_creator_form.update(|f| f.ipi = value);
                                    }
                                />
                                <div class="form-hint">"Format: 1-11 digits (Either IPI or ISNI required)"</div>
                            </div>
                            
                            <div class="form-field">
                                <label for="creator_isni" class="form-label">"ISNI code"</label>
                                <input
                                    type="text"
                                    id="creator_isni"
                                    class="form-input"
                                    placeholder="0000000123456789"
                                    maxlength="16"
                                    value=move || creator_form.get().isni
                                    on:input=move |ev| {
                                        set_creator_form.update(|f| f.isni = event_target_value(&ev));
                                    }
                                />
                                <div class="form-hint">"Format: 16 characters (Either IPI or ISNI required)"</div>
                            </div>
                        </div>
                        
                        // Role selection (badges)
                        <div class="form-field">
                            <label class="form-label">"Roles *"</label>
                            <div class="role-badges">
                                {CREATOR_ROLES.iter().map(|role| {
                                    let role_str = role.to_string();
                                    let role_for_class = role_str.clone();
                                    let role_for_check = role_str.clone();
                                    let role_for_click = role_str.clone();
                                    view! {
                                        <button
                                            type="button"
                                            class=move || {
                                                if creator_form.get().selected_roles.contains(&role_for_class) {
                                                    "role-badge role-badge-selected"
                                                } else {
                                                    "role-badge"
                                                }
                                            }
                                            on:click=move |_| toggle_role(role_for_click.clone())
                                        >
                                            {role_str.clone()}
                                            {move || {
                                                if creator_form.get().selected_roles.contains(&role_for_check) {
                                                    view! { <span class="role-check">" ✓"</span> }.into_view()
                                                } else {
                                                    view! { <></> }.into_view()
                                                }
                                            }}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                        
                        // Add button
                        <div class="form-actions-center">
                            <button
                                type="button"
                                class="btn-add"
                                disabled=move || {
                                    let form = creator_form.get();
                                    form.ipi.is_empty() && form.isni.is_empty() || form.selected_roles.is_empty()
                                }
                                on:click=add_creators
                            >
                                "+ Add Creator"
                            </button>
                        </div>
                        
                        // Creators list (grouped by IPI/ISNI)
                        {move || {
                            let groups = grouped_creators();
                            if groups.is_empty() {
                                view! {
                                    <div class="empty-list">
                                        <p>"No creators assigned yet."</p>
                                        <p class="empty-hint">"Use the form above to add creators."</p>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="creators-list">
                                        <div class="list-header">
                                            <span class="list-title">"Assigned Creators"</span>
                                            <span class="list-count">{groups.len()}" creator(s)"</span>
                                        </div>
                                        <ul class="list-items">
                                            {groups.into_iter().map(|group| {
                                                let pid_display = format_party_id(&group.party_id);
                                                let group_pid = group.party_id.clone();
                                                view! {
                                                    <li class="list-item">
                                                        <div class="item-content">
                                                            <div class="item-id">{pid_display}</div>
                                                            <div class="item-roles">
                                                                {group.roles.iter().map(|role| {
                                                                    view! {
                                                                        <span class="role-tag">{role.clone()}</span>
                                                                    }
                                                                }).collect::<Vec<_>>()}
                                                            </div>
                                                        </div>
                                                        <button
                                                            type="button"
                                                            class="btn-remove"
                                                            on:click=move |_| remove_creator_group(group_pid.clone())
                                                        >
                                                            "✕"
                                                        </button>
                                                    </li>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </div>
                                }.into_view()
                            }
                        }}
                    </div>
                    
                    // PUBLISHERS SECTION (Card style)
                    <div class="ts-section-card">
                        <h3 class="ts-section-title">"Publishers"</h3>
                        <p class="ts-section-subtitle">"Add publishers (IPI or ISNI)"</p>
                        
                        // Publisher form
                        <div class="grid-cols-2">
                            <div class="form-field">
                                <label for="publisher_ipi" class="form-label">"IPI code"</label>
                                <input
                                    type="text"
                                    id="publisher_ipi"
                                    class="form-input"
                                    placeholder="123456789"
                                    inputmode="numeric"
                                    maxlength="11"
                                    value=move || publisher_form.get().ipi
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev).chars()
                                            .filter(|c| c.is_numeric())
                                            .take(11)
                                            .collect::<String>();
                                        set_publisher_form.update(|f| f.ipi = value);
                                    }
                                />
                            </div>
                            
                            <div class="form-field">
                                <label for="publisher_isni" class="form-label">"ISNI code"</label>
                                <input
                                    type="text"
                                    id="publisher_isni"
                                    class="form-input"
                                    placeholder="0000000123456789"
                                    maxlength="16"
                                    value=move || publisher_form.get().isni
                                    on:input=move |ev| {
                                        set_publisher_form.update(|f| f.isni = event_target_value(&ev));
                                    }
                                />
                            </div>
                        </div>
                        
                        // Add button
                        <div class="form-actions-center">
                            <button
                                type="button"
                                class="btn-add"
                                disabled=move || {
                                    let form = publisher_form.get();
                                    form.ipi.is_empty() && form.isni.is_empty()
                                }
                                on:click=add_publisher
                            >
                                "+ Add Publisher"
                            </button>
                        </div>
                        
                        // Publishers list
                        {move || {
                            let pubs = publishers.get();
                            if pubs.is_empty() {
                                view! {
                                    <div class="empty-list">
                                        <p>"No publishers assigned yet."</p>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="creators-list">
                                        <div class="list-header">
                                            <span class="list-title">"Assigned Publishers"</span>
                                            <span class="list-count">{pubs.len()}" publisher(s)"</span>
                                        </div>
                                        <ul class="list-items">
                                            {pubs.iter().enumerate().map(|(idx, pub_)| {
                                                let pid_display = format_party_id(&pub_.id);
                                                view! {
                                                    <li class="list-item">
                                                        <div class="item-content">
                                                            <div class="item-id">{pid_display}</div>
                                                        </div>
                                                        <button
                                                            type="button"
                                                            class="btn-remove"
                                                            on:click=move |_| remove_publisher(idx)
                                                        >
                                                            "✕"
                                                        </button>
                                                    </li>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </div>
                                }.into_view()
                            }
                        }}
                    </div>
                    
                    // CLASSICAL INFO SECTION (Card style)
                    <div class="ts-section-card">
                        <h3 class="ts-section-title">"Classical Information"</h3>
                        <p class="ts-section-subtitle">"Optional fields for classical works"</p>
                        
                        <div class="grid-cols-3">
                            <div class="form-field">
                                <label for="opus" class="form-label">"Opus"</label>
                                <input
                                    type="text"
                                    id="opus"
                                    class="form-input"
                                    placeholder="Op. 27 No. 2"
                                    value=move || opus.get()
                                    on:input=move |ev| set_opus.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div class="form-field">
                                <label for="catalog_number" class="form-label">"Catalog Number"</label>
                                <input
                                    type="text"
                                    id="catalog_number"
                                    class="form-input"
                                    placeholder="BWV 1007"
                                    value=move || catalog_number.get()
                                    on:input=move |ev| set_catalog_number.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div class="form-field">
                                <label for="number_of_voices" class="form-label">"Number of Voices"</label>
                                <input
                                    type="number"
                                    id="number_of_voices"
                                    class="form-input"
                                    placeholder="4"
                                    min="1"
                                    max="99"
                                    value=move || number_of_voices.get()
                                    on:input=move |ev| set_number_of_voices.set(event_target_value(&ev))
                                />
                            </div>
                        </div>
                    </div>
                    
                    // SUBMIT BUTTON
                    <div class="form-submit-section">
                        {move || {
                            if !wallet_connected.get() {
                                view! {
                                    <div class="wallet-warning">
                                        "⚠️ Please connect your wallet to submit."
                                    </div>
                                }.into_view()
                            } else {
                                view! { <></> }.into_view()
                            }
                        }}
                        
                        <button
                            type="submit"
                            class="btn-submit"
                            disabled=move || !wallet_connected.get() || is_submitting.get()
                        >
                            {move || {
                                if is_submitting.get() {
                                    "Submitting...".to_string()
                                } else {
                                    "Review Entries & Submit".to_string()
                                }
                            }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}

// Helper: check if two PartyIds are the same
fn parties_match(a: &PartyId, b: &PartyId) -> bool {
    match (a, b) {
        (PartyId::Ipi(a_ipi), PartyId::Ipi(b_ipi)) => a_ipi == b_ipi,
        (PartyId::Isni(a_isni), PartyId::Isni(b_isni)) => a_isni == b_isni,
        _ => false,
    }
}

// Helper: format PartyId for display
fn format_party_id(pid: &PartyId) -> String {
    match pid {
        PartyId::Ipi(ipi) => format!("IPI: {}", ipi),
        PartyId::Isni(isni) => format!("ISNI: {}", isni),
        // Note: PartyId::Both exists only in backend (non-WASM)
        // but we need to handle it in pattern matching for exhaustiveness
        #[cfg(not(target_arch = "wasm32"))]
        PartyId::Both { ipi, isni } => format!("IPI: {} / ISNI: {}", ipi, isni),
    }
}
