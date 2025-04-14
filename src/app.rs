use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, hooks::use_navigate, StaticSegment
};

use crate::{components::navbar::NavBar, pages::{home::Home, not_found::NotFound, sign_in::SignIn, test::Test}, utils_and_structs::{front_utils::UserState, user_types::UserInfo}};

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


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let user_state = RwSignal::new(UserState::default());
    provide_context(user_state);

    let user_info = RwSignal::new(UserInfo::default());
    provide_context(user_info);

    let handle_unauthenticated_user = move |_| {
        if user_state.get() == UserState::default() {
            return;
        }

        if !user_state.get().is_authenticated() {
            let navigator = use_navigate();
            navigator("/sign-in", Default::default());
        }
    };

    Effect::new(handle_unauthenticated_user);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lex-decks.css"/>

        // sets the document title
        <Title text="Welcome to LexDecks"/>

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
