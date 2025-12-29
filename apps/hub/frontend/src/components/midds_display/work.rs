//! Musical Work MIDDS display component

use leptos::*;
use crate::services::explorer::{MusicalWorkData, CreatorData};
use super::common::{format_party_id, MiddsHeader, MiddsField};

/// Display a single creator with roles
#[component]
fn CreatorItem(
    #[prop(into)] index: usize,
    #[prop(into)] creator: CreatorData,
) -> impl IntoView {
    let id_display = format_party_id(&creator.id);
    
    view! {
        <div class="midds-array-item">
            <span class="midds-index">{index}</span>
            <div class="midds-object">
                <div class="midds-prop">
                    <span class="prop-key">"id"</span>
                    <span class="prop-value ipi-code">{id_display}</span>
                </div>
                <div class="midds-prop">
                    <span class="prop-key">"role"</span>
                    <span class="prop-value">
                        {creator.roles.into_iter().map(|role| {
                            view! { <span class="role-badge">{role}</span> }
                        }).collect::<Vec<_>>()}
                    </span>
                </div>
            </div>
        </div>
    }
}

/// Display musical work in MIDDS format
#[component]
pub fn WorkDisplay(
    #[prop(into)] work: MusicalWorkData,
) -> impl IntoView {
    let creators_count = work.creators.len();
    
    view! {
        <div class="midds-work">
            <MiddsHeader midds_type="MusicalWork"/>
            
            // Title
            <MiddsField label="title">
                {work.title}
            </MiddsField>
            
            // ISWC (optional)
            {work.iswc.map(|iswc| view! {
                <MiddsField label="iswc" class="iswc-value">
                    {iswc}
                </MiddsField>
            })}
            
            // Creators
            <div class="midds-field">
                <div class="midds-label">
                    "creators" 
                    <span class="midds-count">"(" {creators_count} ")"</span>
                </div>
                <div class="midds-array">
                    <For
                        each=move || work.creators.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(index, creator)| {
                            view! { <CreatorItem index=index creator=creator/> }
                        }
                    />
                </div>
            </div>
            
            // Creation Year (optional)
            {work.creation_year.map(|year| view! {
                <MiddsField label="creationYear">
                    {year}
                </MiddsField>
            })}
            
            // Work Type
            <MiddsField label="workType" class="type-value">
                {work.work_type}
            </MiddsField>
            
            // Instrumental
            <MiddsField label="instrumental" class="bool-value">
                {if work.is_instrumental { "true" } else { "false" }}
            </MiddsField>
            
            // Language (optional)
            {work.language.map(|lang| view! {
                <MiddsField label="language">
                    {lang.clone()}
                    <span class="midds-lang">{lang}</span>
                </MiddsField>
            })}
            
            // Musical Key (optional)
            {work.musical_key.map(|key| view! {
                <MiddsField label="key">
                    {key}
                </MiddsField>
            })}
        </div>
    }
}

