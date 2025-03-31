use leptos::{html::Img, prelude::*};

use crate::utils_and_structs::{db_and_cache::clear_cache, front_utils::UserState, shared_truth::{LOCAL_AUTH_TOKEN_KEY, LOCAL_REFRESH_TOKEN_KEY, LOCAL_USER_INFO_KEY}, user_types::UserInfo};

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
