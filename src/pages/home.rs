use leptos::prelude::*;

use crate::components::message_box::MessageBox;
use crate::components::study_window::{StudyWindow, StudyType};


/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <MessageBox/>
        <div class="content-flex home-container">
            <div class="multi-or-one-grid">
                <StudyWindow study_type=StudyType::Lesson/>
                <StudyWindow study_type=StudyType::Review/>
            </div>
            <div class="ka chow black-bg">
            </div>
            <div class="calender giver blue-bg">
            </div>
        </div>
    }
}
