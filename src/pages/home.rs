use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::prelude::*;

use crate::components::message_box::MessageBox;


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
            ..Default::default()
        },
        StudyType::Review => ButtonConfig {
            text: "Start Reviewing".to_string(),
            background_color: Color::Winter4,
            border_color: Color::Winter4,
            ..Default::default()
        },
    };

    let window_image = match study_type {
        StudyType::Lesson => "..\\..\\images\\Transparent_LexLingua_Logomark.png",
        StudyType::Review => "..\\..\\images\\Transparent_LexLingua_Primary Logo.png",
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

// .sign-in-link {
// 	box-sizing: border-box;
// 	width: var(--sign-in-link-and-logo-nav-width);
// 	transition: all 0.3s ease 0s;
// 	font-family: var(--font-family-default);
// 	font-size: var(--sign-in-link-and-logo-nav-font-size);
// 	text-decoration: none;
// 	text-align: center;
// 	border-radius: 3px;
// 	padding: 0.9svh 0;
// 	border: 1px solid var(--winter3);
// 	background-color: var(--winter3);
// 	color: var(--white);
// 	font-weight: 600;
// 	text-shadow: var(--text-shadow-dark);
// }

// .sign-in-link:hover {
// 	text-decoration: none;
// 	outline: none;
// 	outline-width: 0;
// 	border: 1px solid var(--white);
// 	box-shadow: 0 1px 1px var(--white);
// 	cursor: pointer;
// }

#[component]
pub fn Button(config: ButtonConfig) -> impl IntoView {

    static BUTTON_ID: AtomicUsize = AtomicUsize::new(0);
    let id = BUTTON_ID.fetch_add(1, Ordering::Relaxed);

    let mut this_button = "button".to_string();
    this_button.push_str(&id.to_string());

    let bg = config.background_color.hex();
    let border_col = config.border_color.hex();
    let text_col = config.text_color.hex();

    let this_button_styles = format!("
    .{} {{
        line-height: calc({} - var(--button-border-width) * 2);
        background-color: {};
        border-color: {};
        color: {};
    }}
    .{}:hover {{
        background-color: {};
        border-color: {};
        color: {};
    }}
    ", this_button, config.css_height,
     bg, border_col, text_col,
    this_button, 
    bg, config.text_color.rgba(30), text_col);

    let button_styles = "
        :root {
            --button-padding: 0.5svw;
            --button-border-width: calc(var(--button-padding)/2);
        }
        .button {
            display: block;
            box-sizing: border-box;
            transition: all 0.3s ease 0s;
            text-decoration: none;
            text-align: center;
            border-radius: 3px;
            padding: var(--button-padding);
            border-width: var(--button-border-width);
            border-style: solid;
        }

        .buttonwhat_da {
            background-color: red;
        }

        .button:hover {
            text-decoration: none;
            outline: none;
            outline-width: 0;
            cursor: pointer;
        }
    ";

    let mut classes = this_button.clone();
    classes.push(' ');
    classes.push_str("button");
    let mut font_weight = "400";
    config.bold.then(|| font_weight = "600");
    view! {
        <style>
            {button_styles}
        </style>
        <style>
            {this_button_styles}
        </style>
        <a class=classes style:font-weight=font_weight style:box-shadow=config.box_shadow.css() style:text-shadow=config.text_shadow.css() style:width=config.css_width style:height=config.css_height>
            {config.text}
        </a>
    }
}

pub struct ButtonConfig {
    css_width: String,
    css_height: String,
    text: String,
    text_color: Color,
    background_color: Color,
    border_color: Color,
    text_shadow: Shadow,
    box_shadow: Shadow,
    bold: bool,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        let mut text_shadow = Shadow::new(Color::MidnightBlack, "0", "1px", "0");
        text_shadow.color_intensity = 25;
        
        let mut box_shadow = Shadow::new(Color::Winter2, "0", "1px", "1px");
        box_shadow.color_intensity = 60;
        box_shadow.spread_radius = "".to_string();

        Self { 
            css_width: "auto".to_string(), 
            css_height: "auto".to_string(),
            text: Default::default(), 
            text_color: Color::White, 
            background_color: Color::Winter3, 
            border_color: Color::Winter3, 
            text_shadow, 
            box_shadow,
            bold: true,
        }
    }
}

#[derive(Clone)]
pub struct Shadow {
    color: Color,
    color_intensity: u8,
    inset: bool,
    horizontal_offset: String,
    vertical_offset: String,
    blur_radius: String,
    spread_radius: String,
    shadow: Option<Box<Shadow>>,
}

impl Shadow {
    pub fn new(color: Color, horizontal_offset: impl ToString, vertical_offset: impl ToString, blur_radius: impl ToString) -> Self {

        Self { 
            color,
            horizontal_offset: horizontal_offset.to_string(),
            vertical_offset: vertical_offset.to_string(),
            blur_radius: blur_radius.to_string(),
            ..Default::default()
        }
    }

    pub fn light() -> Self {
        let mut light_shadow = Shadow::new(Color::Winter2, "2px", "2px", "1px");
        light_shadow.color_intensity = 60;
        return light_shadow;
    }

    
    pub fn dark() -> Self {
        let mut dark_shadow = Shadow::new(Color::MidnightBlack, 0, "1px", "1px");
        dark_shadow.color_intensity = 40;
        return dark_shadow;
    }

    pub fn css(&self) -> String {
        let mut css = if self.inset {
            "inset ".to_string()
        } else {
            "".to_string()
        };
        css.push_str(&format!("{} {} {} {}",self.horizontal_offset, self.vertical_offset, self.blur_radius, self.color.rgba(self.color_intensity)));
        
        let shadow = match self.shadow.clone() {
            Some(bubububox) => *bubububox,
            None => return css,
        };

        css.push_str(", ");
        css.push_str(&shadow.css());
        return css;
    }
}

impl Default for Shadow {
    fn default() -> Self {
        Self { 
            color: Default::default(), 
            color_intensity: Default::default(), 
            inset: Default::default(),
            horizontal_offset: Default::default(),
            vertical_offset: Default::default(),
            blur_radius: Default::default(),
            spread_radius: Default::default(),
            shadow: Default::default(),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Color {
    Mint,
    MidnightBlack,
    FrenchGray,
    Winter1,
    Winter2,
    Winter3,
    Winter4,
    Red,
    LightGray,
    OffWhite,
    DarkSlate,
    #[default]
    White,
}

impl Color {
    pub fn hex(&self) -> String {
        let hex = match self{
            Color::Mint => "#15F5BA",
            Color::MidnightBlack => "#25282B",
            Color::FrenchGray => "#B5BEC6",
            Color::Winter1 => "#eee",
            Color::Winter2 => "#96EFFF",
            Color::Winter3 => "#5FBDFF",
            Color::Winter4 => "#7B66FF",
            Color::Red => "#FF0A23",
            Color::LightGray => "#eee",
            Color::OffWhite => "#F4F4F4",
            Color::DarkSlate => "#416163",
            Color::White => "#ffffff",
        };
        hex.to_string()
    }

    pub fn rgb(&self) -> String {
        let rgb = match self {
            Color::Mint => "21, 245, 186",
            Color::MidnightBlack => "37, 40, 43",
            Color::FrenchGray => "181, 190, 198",
            Color::Winter1 => "197, 255, 248",
            Color::Winter2 => "150, 239, 255",
            Color::Winter3 => "95, 189, 255",
            Color::Winter4 => "123, 102, 255",
            Color::Red => "255, 10, 35",
            Color::LightGray => "238, 238, 238",
            Color::OffWhite => "244, 244, 244",
            Color::DarkSlate => "65, 97, 99",
            Color::White => "255, 255, 255",
        };
        rgb.to_string()
    }

    pub fn rgba(&self, percentage: u8) -> String {
        let intensity = if percentage > 100 {
            eprintln!("Percentage must be between 1, 100");
            100
        } else {
            percentage
        };

        let intensity = intensity as f32 / 100.0;

        return format!("rgba({}, {})", self.rgb(), intensity);
    }
}
