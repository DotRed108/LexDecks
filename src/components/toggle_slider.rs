use leptos::{html::Input, prelude::*};

use crate::utils_and_structs::ui::Color;

#[component]
pub fn SlideToggleCheckbox(checkbox_ref: NodeRef<Input>, #[prop(default = "remember".into())] action_form_name: String) -> impl IntoView {

    let height = "30px";
    let styles = format!("
    .sign-in-remember-input-container {{
        font-family: var(--font-family-default);
        font-size: min(1em, 3dvmin);
        align-self: baseline;
        color: {darkslate};
        accent-color: {winter3};
    }}
    .sign-in-remember-label {{
        align-self: baseline;
        display: flex;
        flex-direction: row;
        align-items: center;
    }}
    .sign-in-remember-input {{
        accent-color: {winter3};
        cursor: pointer;
        transform: scale(1.0);
    }}
    .remember-input-slide-toggle {{
        display: block;
        position: relative;
        flex: none;
        width: 50px;
        height: {height};
        border-radius: {height};
        background-color: {light_gray};
        cursor: pointer;
        transition: all 0.1s ease-in-out;
        z-index: 1;

        &::before,
        &::after {{
            content: ' ';
            display: block;
            position: absolute;
            top: 1px;
            border-radius: {height};
            height: calc({height} - 3px);
            background-color: {light_gray};
            transform: translate3d(0,0,0);
            transition: 0.2s cubic-bezier(0, 1.1, 1, 1.1);;
        }}

        &::before {{
            z-index: -1;
            width: 48px;
            right: 1px;
            transform: scale(1);
        }}

        &::after {{
            z-index: 1;
            width: 28px;
            left: 1px;
            box-shadow: 0 1px 4px 0.5px rgba(0, 0, 0, 0.25);
        }}

        input:checked + & {{
            background-color: {winter3};

            &::before {{
                transform: scale(0);
            }}

            &::after {{
                transform: translate3d(20px,0,0);
            }}
        }}  
    }}
    ", darkslate = Color::DarkSlate.hex(),
        winter3 = Color::Winter3.hex(),
        light_gray = Color::LightGray.hex(),
    );

    view! {
        <style>{styles}</style>
        <label style:user-select="none" class="sign-in-remember-label" for="remember">
            <input style:display="none" class="sign-in-remember-input" id="remember" value="true" name=action_form_name node_ref=checkbox_ref checked type="checkbox"/>
            <div id="toggle" class="remember-input-slide-toggle"></div>
            <label for="remember">"\u{00A0}Remember Me"</label>
        </label>
    }
}
