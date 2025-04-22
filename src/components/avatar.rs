use leptos::{html::Img, prelude::*};

use crate::utils::{shared_utilities::UserState, user_types::UserInfo};

#[component]
pub fn ThisUserAvatar() -> impl IntoView {
    let _user_info = use_context::<RwSignal<UserInfo>>().unwrap_or_default();
    let _user_state = use_context::<RwSignal<UserState>>().unwrap_or_default();


    let img_ref = NodeRef::<Img>::new();

    let log_out =move |_ev| {
    };

    let pfp_as_src = move || {
        "".to_string()
    };

    

    view! {
        <img /*on:error=move |_| pfp_resource.refetch()*/ on:click=log_out class="header-pfp" node_ref=img_ref src=pfp_as_src/>
    }
}
