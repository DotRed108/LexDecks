use leptos::prelude::*;

use crate::{components::{avatar::ThisUserAvatar, button::{Button, ButtonConfig, ButtonType}}, utils_and_structs::{shared_utilities::UserState, shared_truth::LOGO_PATH, ui::Color}};

#[component]
pub fn NavBar() -> impl IntoView {
    let user_state = expect_context::<RwSignal<UserState>>();
    // tuple is (name, link)
    let navbar = [("Home", "/"), ("Create Deck", "/create-deck"), ("Kanji", "#"), ("Vocabulary", "#")];

    let no_auth_navlist = || view! {
        <h1 style:margin="0" style:font-size="1.8em">"LexLingua"</h1>
    };

    let sign_in_button = || {
        let sign_in_button_config = ButtonConfig {
            text: "Sign In".to_string(),
            button_type: ButtonType::Link("/sign-in"),
            text_color: Color::White,
            border_color: Color::Winter3,
            background_color: Color::Winter3,
            id: "nav_sign_in".into(),
            ..Default::default()
        };
        view! {
            <Button config=sign_in_button_config/>
        }
    };

    view! {
        <header class="global-header full-width">
            <nav class="navbar">
                <a class="logo-navigator" href="/"><img class="nav-logo" src=LOGO_PATH alt="lex logo"/></a>
                <ol class = "navlist">
                    <Show when=move || {user_state.get().is_authenticated()} fallback=no_auth_navlist>
                    {navbar.into_iter().map(|(name, link)| 
                        view! {
                            <li class={format!("navlist-element {}-nav", name.to_lowercase())}>
                                <a class="navlist-link" href={link}>{name}</a>
                            </li>
                        }
                    ).collect_view()}
                    </Show>
                </ol>
                <Show when=move || {user_state.get().is_authenticated()} fallback=sign_in_button>
                    <ThisUserAvatar/>
                </Show>
            </nav>
        </header>
    }
}