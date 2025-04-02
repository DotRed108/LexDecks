use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};

#[component]
pub fn SignIn() -> impl IntoView {
    #[server]
    pub async fn shouting_text(input: String) -> Result<String, ServerFnError> {
        println!("ran");
        Ok(input.to_ascii_uppercase())
    }
    let action = ServerAction::<ShoutingText>::new();

    let what_da = move |_| spawn_local(async move {
        action.dispatch("what da".to_string().into());

        console_log("what da");
    });

    view! {
        <div on:click=what_da>
            "hi nigger"
            {move || action.value().get()}
        </div>
    }
}
