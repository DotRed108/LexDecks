use leptos::prelude::*;
use crate::components::calendar::Calendar;
use crate::components::message_box::MessageBox;
use crate::components::study_window::{StudyWindow, StudyType};
use crate::utils_and_structs::database_types::{DeckId, DeckList};

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    let current_deck = RwSignal::new(DeckId::default());
    let subject = RwSignal::new("Welcome to LexDecks! Here are your reviews and lessons for the day.".to_string());
    let urgent = RwSignal::new(false);
    let message = RwSignal::new("gay".to_string());
    view! {
        <MessageBox subject urgent message margin_top="var(--default-div-margin)".into()/>
        <div class="content-flex home-container">
            <div class="multi-or-one-grid">
                <StudyWindow study_type=StudyType::Lesson/>
                <StudyWindow study_type=StudyType::Review/>
            </div>
            <DeckSelector _deck_list = deck_list/>
            <Calendar current_deck/>
        </div>
    }
}

#[component]
pub fn DeckSelector(_deck_list: DeckList) -> impl IntoView {
    view! {
        <form class="ka chow giver black-bg">
        </form>
    }
}