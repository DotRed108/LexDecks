use crate::utils_and_structs::{user_types::UserInfo, shared_truth::{LOCAL_AUTH_TOKEN_KEY, LOCAL_REFRESH_TOKEN_KEY, LOCAL_USER_INFO_KEY, LOGO_PATH}};
use leptos::{html::Img, prelude::*};
use crate::utils_and_structs::{db_and_cache::clear_cache, front_utils::UserState};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="global-header full-width full-width-no-inherit">
            <NavBar/>
        </header>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let user_state = expect_context::<RwSignal<UserState>>();
    // tuple is (name, link)
    let navbar = [("Home", "/"), ("Create Deck", "/create-deck"), ("Kanji", "#"), ("Vocabulary", "#")];

    let no_auth_navlist = || view! {
        <h1>"LexLingua"</h1>
    };
    let sign_in = || view! {
        <a class="sign-in-link" href="/sign-in">"Sign In"</a>
    };
    view! {
        <a class="logo-navigator" href="/"><img class="nav-logo" src=LOGO_PATH alt="lex logo"/></a>
        <nav class="navbar">
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
        </nav>
        <Show when=move || {user_state.get().is_authenticated()} fallback=sign_in>
        <ThisUserAvatar/>
        </Show>
    }
}

#[component]
pub fn ThisUserAvatar() -> impl IntoView {
    let user_info = use_context::<RwSignal<UserInfo>>().unwrap_or_default();
    let user_state = use_context::<RwSignal<UserState>>().unwrap_or_default();


    let img_ref = NodeRef::<Img>::new();

    let log_out =move |_ev| {
        clear_cache(LOCAL_REFRESH_TOKEN_KEY).unwrap();
        clear_cache(LOCAL_AUTH_TOKEN_KEY).unwrap();
        user_state.set(UserState::default());
        clear_cache(LOCAL_USER_INFO_KEY).unwrap();
        user_info.set(UserInfo::default());
    };

    let pfp_as_src = move || {
        "".to_string()
    };

    

    view! {
        <img /*on:error=move |_| pfp_resource.refetch()*/ on:click=log_out class="header-pfp" node_ref=img_ref src=pfp_as_src/>
    }
}
