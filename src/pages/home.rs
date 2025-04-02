use leptos::prelude::*;
use crate::components::calendar::Calendar;
use crate::components::message_box::MessageBox;
use crate::components::study_window::{StudyWindow, StudyType};
use crate::utils_and_structs::database_types::DeckId;

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    let current_deck = RwSignal::new(DeckId::default());
    view! {
        <MessageBox/>
        <div class="content-flex home-container">
            <div class="multi-or-one-grid">
                <StudyWindow study_type=StudyType::Lesson/>
                <StudyWindow study_type=StudyType::Review/>
            </div>
            <DeckSelector/>
            <Calendar current_deck/>
        </div>
    }
}

#[component]
pub fn DeckSelector() -> impl IntoView {
    view! {
        <div class="ka chow giver black-bg">
        </div>
    }
}