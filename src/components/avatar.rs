use leptos::{html::Img, prelude::*};

#[component]
pub fn ThisUserAvatar() -> impl IntoView {
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
