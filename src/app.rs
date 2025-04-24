use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, StaticSegment
};

use crate::{components::navbar::NavBar, pages::{home::Home, not_found::NotFound, sign_in::SignIn, test::Test}, utils::user_types::setup_user};

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
    setup_user();
    

    view! {
        <Stylesheet id="leptos" href="/pkg/lex-decks.css"/>

        <Title text="LexLingua"/>

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
