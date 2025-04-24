use leptos::prelude::*;
use crate::{
    components::{button::{Button, ButtonConfig, ButtonType}, 
    message_box::MessageBox}, 
    utils::{
        shared_truth::FULL_LOGO_PATH, 
        ui::{Color, Shadow},
        user_types::{sign_out, UserState},
    }
};

#[component]
pub fn SignOut() -> impl IntoView {
    let user_resource = expect_context::<Resource<UserState>>();
    let user_state = expect_context::<RwSignal<UserState>>();

    sign_out(user_state, user_resource);

    Effect::new(move |_|
        sign_out(user_state, user_resource)
    );
    
    let sign_out_subject = RwSignal::new("You have been signed out".into());
    let sign_out_urgent = RwSignal::new(false);
    let sign_out_message = RwSignal::new(String::new());

    let sign_in_height = "min(var(--sign-in-element-min-height), var(--sign-in-element-max-height))";
    let shadow_size = "min(calc(5svmax - 5svh), 15px)";
    let email_input_width = "100%";
    let mut box_shadow = Shadow::new(Color::Winter2, "0", "1px", "1px");
    box_shadow.color_intensity = 60;
    box_shadow.spread_radius = "".to_string();
    let styles = 
        format!("
        :root {{
            --sign-in-container-padding: calc(6svmax - 6svh);
            --sign-in-element-max-height: 42px;
            --sign-in-element-min-height: 10svmin;
            --sign-in-element-height: {sign_in_height};
            --sign-in-container-gap: calc(1svmax + 2svh);
        }}
        .sign-out-container {{
            position: relative;
            overflow: hidden;
            width: min(98%, 30em);
            place-self: start;
            justify-self: center;
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;
            padding: var(--sign-in-container-padding);
            padding-top: calc(var(--sign-in-container-padding)/2);
            gap: var(--sign-in-container-gap);
            border-radius: 10px;
            margin-top: max(var(--default-div-margin) + {shadow_size}, 25svh - 25svw + {shadow_size});
            margin-bottom: {shadow_size};
            position: relative;
            box-shadow: {surround};
        }}
        .sign-in-logo {{
            height: auto;
            padding: 0 12%;
            max-width: 100%;
            object-fit: contain;
        }}",
        surround = Shadow::surrounding_shadow(Color::Winter2, shadow_size).css(),
    );

    view! {
        <style>{styles}</style>
        <div class="sign-out-container">
            <img src=FULL_LOGO_PATH alt="LexLinguaLogo" class="sign-in-logo"/>
            <MessageBox subject=sign_out_subject urgent=sign_out_urgent message=sign_out_message width=email_input_width.into() only_subject=true top_padding="calc(var(--sign-in-element-height)/2 - 0.5em)".into()/>
            <Button config=ButtonConfig {id:"goback".into(), button_type: ButtonType::Link("/sign-in"),css_height: sign_in_height.into(), text:"Go Back".into(), css_width: email_input_width.into(), ..Default::default()}/>
        </div>
    }
}