use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::prelude::*;

use crate::utils_and_structs::ui::{Color, Shadow};

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
    pub css_width: String,
    pub css_height: String,
    pub text: String,
    pub text_color: Color,
    pub background_color: Color,
    pub border_color: Color,
    pub text_shadow: Shadow,
    pub box_shadow: Shadow,
    pub bold: bool,
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