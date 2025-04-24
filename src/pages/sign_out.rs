use leptos::{component, view, IntoView};

use crate::pages::sign_in::SignIn;

#[component]
pub fn SignOut() -> impl IntoView {
    view! {
        <SignIn/>
    }
}