use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, StaticSegment
};

use crate::{components::navbar::NavBar, pages::{home::Home, not_found::NotFound, sign_in::SignIn, test::Test}, utils::{shared_utilities::{initial_user_state, UserState}, user_types::UserInfo}};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="content-grid">
                <App/>
            </body>
        </html>
    }
}


#[derive(Clone, Copy, PartialEq)]
pub enum UpdateUserState {
    Clear,
    Fetch,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let user_action = Action::new_with_value(Some(initial_user_state()),
        |update_type: &UpdateUserState| {
            let update_type = *update_type;
            async move {
                match update_type {
                    UpdateUserState::Clear => UserState::default(),
                    UpdateUserState::Fetch => UserState::find_token_or_default().await,
                }
            }
        }
    );

    // Effect::new(move || {user_action.dispatch(UpdateUserState::Fetch);});

    provide_context(user_action);

    let user_info = RwSignal::new(UserInfo::default());
    provide_context(user_info);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lex-decks.css"/>

        // sets the document title
        <Title text="LexLingua"/>

        // content for this welcome page
        <Router>
            <NavBar/>
            <Routes fallback=|| NotFound>
                <Route path=StaticSegment("") view=Home/>
                <Route path=StaticSegment("/sign-in") view=SignIn/>
                <Route path=StaticSegment("/test") view=Test/>
            </Routes>
        </Router>
    }
}
