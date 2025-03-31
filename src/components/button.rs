use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::{either::Either, prelude::*};

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
    let padding = config.padding;

    let mut this_button_styles = format!("
    .{this_button} {{
        line-height: calc({height} - var(--button-border-width) * 2);
        background-color: {bg};
        border-color: {border_col};
        color: {text_col};
        padding: {padding};
        border-width: {border_width};
    }}
    .{this_button}:hover {{
        background-color: {bg};
        border-color: {rgba_border_col};
        color: {text_col};
        padding: {padding};
        border-width: {border_width};
    }}
    ", height=config.css_height, 
    border_width=format!("calc({padding}/2)"),
    rgba_border_col=config.text_color.rgba(30));

    let button_styles = "
        :root {
            --button-border-width: calc(var(--button-padding)/2);
        }
        .button {
            display: block;
            box-sizing: border-box;
            transition: all 0.3s ease 0s;
            text-decoration: none;
            text-align: center;
            border-radius: 3px;
            border-style: solid;
            font-size: 1em;
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
    this_button_styles.push_str(button_styles);

    let mut classes = this_button.clone();
    classes.push(' ');
    classes.push_str("button");
    let mut font_weight = "400";
    config.bold.then(|| font_weight = "600");
    
    view! {
        <style>{this_button_styles}</style>
        {match config.link {
            Some(link) => Either::Left(
                view! {
                    <a class=classes href=link style:font-weight=font_weight style:box-shadow=config.box_shadow.css() style:text-shadow=config.text_shadow.css() style:width=config.css_width style:height=config.css_height>
                        {config.text}
                    </a>
                }
            ),
            None => Either::Right(
                view! {
                    <button class=classes style:font-weight=font_weight style:box-shadow=config.box_shadow.css() style:text-shadow=config.text_shadow.css() style:width=config.css_width style:height=config.css_height>
                        {config.text}
                    </button>
                }
            ),
        }}
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
    pub link: Option<String>,
    pub padding: String,
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
            link: None,
            padding: "calc(0.6ch + 0.3svw)".to_string(),
        }
    }
}