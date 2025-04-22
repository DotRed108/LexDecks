use leptos::prelude::*;

use crate::{components::button::{Button, ButtonConfig}, utils::{shared_truth::{LESSONS_IMAGE, REVIEW_IMAGE}, ui::{Color, Shadow}}};

#[component]
pub fn StudyWindow(study_type: StudyType) -> impl IntoView {
    let window_text = match study_type {
        StudyType::Lesson => "All of today's new cards.",
        StudyType::Review => "All of the current cards ready for review.",
    };
    let window_title = match study_type {
        StudyType::Lesson => "Lessons",
        StudyType::Review => "Reviews",
    };

    let hello = match study_type {
        StudyType::Lesson => ButtonConfig {
            text: "Start Learning".to_string(),
            background_color: Color::Mint,
            border_color: Color::Mint,
            text_color: Color::DarkSlate,
            box_shadow: Shadow::dark(),
            padding: "1.7ch".to_string(),
            ..Default::default()
        },
        StudyType::Review => ButtonConfig {
            text: "Start Reviewing".to_string(),
            background_color: Color::Winter4,
            border_color: Color::Winter4,
            box_shadow: Shadow::dark(),
            padding: "1.7ch".to_string(),
            ..Default::default()
        },
    };

    let window_image = match study_type {
        StudyType::Lesson => LESSONS_IMAGE,
        StudyType::Review => REVIEW_IMAGE,
    };

    let styles = format!("
    .study-window {{
        --gap: calc(0.5svw + 1.4svh);
        height: auto;
        gap: var(--gap);
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        padding: var(--gap);
        border-radius: 6px;
        box-shadow: {light};
        /* max-height: 15em; currently overflows at certain browser sizes */
    }}
    .lesson-window {{
        background-color: {winter4};
        color: {white};
    }}
    .review-window {{
        background-color: {mint};
    }}
    .study-window-main {{
        --grower-min-width: 10px;
        --giver-width: 10px;
        --gap: 2.5svmin;
        width: 100%;
        height: 100%;
    }}
    .study-window-title-description-container {{
        display: flex;
        flex-direction: column;
        gap: 0.5em;
        height: 100%;
    }}
    .study-window-title {{
        font-size: 24px;
        font-weight: 600;
    }}
    .study-window-description {{
        font-size: 16px;
        font-weight: 400;
    }}
    .study-window-image {{
        max-width: 34%;
        object-fit: contain;
    }}",
    light=Shadow::light().css(),
    white=Color::White.hex(),
    winter4=Color::Winter4.hex(), 
    mint=Color::Mint.hex());

    let window_classes = match study_type {
        StudyType::Lesson => "study-window lesson-window",
        StudyType::Review => "study-window review-window",
    };

    view! {
        <style>{styles}</style>
        <div class=window_classes>
            <div class="content-flex study-window-main">
                <img class="study-window-image giver" src=window_image/>
                <div class="study-window-title-description-container">
                    <h2 class="study-window-title">{window_title}</h2>
                    <p class="study-window-description">
                        {window_text}
                    </p>
                </div>
            </div>
            <Button config=hello/>
        </div>
    }
}

#[derive(Clone, Copy)]
pub enum StudyType {
    Lesson,
    Review,
}