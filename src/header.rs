use crate::components::navbar::NavBar;
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="global-header full-width full-width-no-inherit">
            <NavBar/>
        </header>
    }
}
