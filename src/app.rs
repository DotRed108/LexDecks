use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, StaticSegment
};

use crate::{components::navbar::NavBar, pages::{home::Home, not_found::NotFound, sign_in::SignIn, test::Test}, utils_and_structs::{shared_utilities::UserState, user_types::UserInfo}};

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
    Reset,
    Fetch,
}

// #[derive(Clone, Debug)]
// pub struct GlobalUserResource {
//     dispatcher: RwSignal<UpdateUserState>,
//     resource: Resource<UserState>
// }

// impl GlobalUserResource {
//     fn new() -> GlobalUserResource {
//         let dispatcher = RwSignal::new(UpdateUserState::Reset);
//         GlobalUserResource { 
//             dispatcher, 
//             resource: Resource::new_blocking(move || dispatcher.get(), |_hi| UserState::find_token_or_default()),
//         }
//     }
// }

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    // let user_state = Resource::new_blocking(move || (), |_| async {UserState::find_token_or_default().await});
    let user_action = Action::new(
        |update_type: &UpdateUserState| {
            let update_type = *update_type;
            async move {
                match update_type {
                    UpdateUserState::Reset => UserState::default(),
                    UpdateUserState::Fetch => UserState::find_token_or_default().await,
                }
            }
        }
    );

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
