use std::time::Duration;

use leptos::{either::Either, prelude::*};
#[cfg(feature = "ssr")]
use tokio::time::sleep;

use crate::{components::{button::{Button, ButtonConfig, ButtonType}, message_box::MessageBox, toggle_slider::SlideToggleCheckbox}, utils_and_structs::{outcomes::Outcome, shared_truth::{FULL_LOGO_PATH, MAX_EMAIL_SIZE}, ui::{Color, Shadow} }};

#[component]
pub fn SignIn() -> impl IntoView {
    let subject = RwSignal::new(String::new());
    let urgent = RwSignal::new(false);
    let message = RwSignal::new(String::new());

    let input_element = NodeRef::new();
    let checkbox_element = NodeRef::new();

    #[server]
    pub async fn send_email(email: String) -> Result<Outcome, ServerFnError> {
        println!("What da");

        sleep(Duration::from_secs(1)).await;
        if email == "whatda@gmail.com".to_string() {
            Ok(Outcome::EmailAlreadyInUse)
        } else {
            Ok(Outcome::EmailSendSuccess)
        }
    }

    let request_email_send = ServerAction::<SendEmail>::new();

    let response = request_email_send.value();

    let sign_in_height = "min(var(--sign-in-element-min-height), var(--sign-in-element-max-height))";
    let shadow_size = "min(calc(5svmax - 5svh), 15px)";
    let email_input_width = "100%";
    let styles = 
        format!("
        :root {{
            --sign-in-container-padding: calc(6svmax - 6svh);
            --sign-in-element-max-height: 42px;
            --sign-in-element-min-height: 10svmin;
            --sign-in-element-height: {sign_in_height};
            --sign-in-container-gap: calc(1svmax + 2svh);
        }}
        form {{
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
        }}
        .sign-in-email-input {{
            --input-color: {lightgray};
            color: {darkslate};
            font-family: var(--font-family-default);
            font-size: min(var(--sign-in-element-pixel-size), var(--sign-in-element-font-size));
            border: 1px solid var(--input-color);
            background-color: var(--input-color);
            text-decoration: none;
            text-align: center;
            border-radius: 3px;
            height: {sign_in_height};
            width: {email_input_width};
            max-height: var(--sign-in-element-max-height);
        }}
        .sign-in-email-input:hover {{
            text-decoration: none;
            outline: none;
            outline-width: 0;
            box-shadow: 0 1px 1px var(--mint)
        }}

        .sign-in-email-input:focus {{
            text-decoration: none;
            outline: none;
            outline-width: 0;
            box-shadow: 0 1px 1px var(--mint);
        }}
        .sign-in-button {{
            --capital-u-size: 14px;
            --lowercase-p-size: 10px;
            --capital-i-size: 7px;
            --lowercase-n-size: 10px;
            --letter-spacing: 0.04em;
            overflow: hidden;
            position: relative;
            letter-spacing: var(--letter-spacing);
        }}
        .sign-in-button::before {{
            display: grid;
            align-content: center;
            justify-content: left;
            height: var(--sign-in-element-height);
            border: 1px solid rgba(var(--off-white-rgb), 0);
            background-color: rgba(var(--off-white-rgb), 0);
            position: absolute;
            left: calc(50% + 0.8ch);
            top: 50%;
            transform: translateY(-50%);
            content: \"\";
            overflow: hidden;
            white-space: nowrap;
            margin: 0 auto;
            animation-name: type-in-up;
            animation-duration: 8s;
            animation-iteration-count: infinite;
            animation-timing-function: step-end;
        }}
        @keyframes type-in-up {{
            0% {{
                width: 0;
                content: \"In\";
            }}
            7% {{
                width: var(--capital-i-size);
                content: \"In\";
            }}
            10% {{
                width: calc(var(--capital-i-size) + var(--letter-spacing) + var(--lowercase-n-size));
                content: \"In\";
            }}
            48% {{
                width: var(--capital-i-size);
                content: \"In\";
            }}
            50% {{
                width: 0;
                content: \"In\";
            }}
            57% {{
                width: var(--capital-u-size);
                content: \"Up\";
            }}
            60% {{
                width: calc(var(--capital-u-size) + var(--letter-spacing) + var(--lowercase-p-size));
                content: \"Up\";
            }}
            98% {{
                width: var(--capital-u-size);
                content: \"Up\";
            }}
            100% {{
                width: 0;
                content: \"Up\";
            }}
        }}",
        surround = Shadow::surrounding_shadow(Color::Winter1, shadow_size).css(),
        darkslate = Color::DarkSlate.hex(),
        lightgray = Color::LightGray.hex(),
    );

    let loading_button = move || {
        view! {
            <Button config=ButtonConfig {button_type: ButtonType::Default, text: "Loading".into(), css_height:sign_in_height.into(), css_width: email_input_width.into(), ..Default::default()}/>
        }
    };

    let display_response = move |outcome: Outcome| {
        match outcome {
            Outcome::EmailSendSuccess => {
                subject.set("Check Your Email".into());
                urgent.set(false);
                message.set(String::new());
            },
            Outcome::EmailSendFailure(_) => {
                subject.set("Could not send email. Try again in a bit.".into());
                urgent.set(true);
                message.set(String::new());
            },
            Outcome::EmailAlreadyInUse => {
                subject.set("Email could not be used to sign up as it is already in use.".into());
                urgent.set(true);
                message.set(String::new());
            },
            Outcome::VerificationFailure => {
                subject.set("User could not be verified".into());
                urgent.set(true);
                message.set(String::new());
            },
            Outcome::UserSuspended(date) => {
                subject.set("This email is associated with a suspended account.".into());
                urgent.set(true);
                message.set(String::new());
            },
            _any_other_outcome => {
                subject.set("Unspecified error occured. Please wait and try again.".into());
            }
        }
    };

    let go_back = move |_| {
        response.set(None)
    };

    view! {
        <style>{styles}</style>
        <ActionForm action=request_email_send>
            <img src=FULL_LOGO_PATH alt="LexLinguaLogo" class="sign-in-logo"/>
            // `title` matches the `title` argument to `add_todo`
            <Show when=move || !request_email_send.pending().get() fallback=loading_button>{
                match response.get() {
                    Some(result) => {
                        let outcome = match result {
                            Ok(outcome) => outcome,
                            Err(e) => Outcome::EmailSendFailure(e.to_string())   
                        };
                        display_response(outcome);
                        Either::Left(view! {
                            <MessageBox subject urgent message width=email_input_width.into() only_subject=true top_padding="calc(var(--sign-in-element-height)/2 - 0.5em)".into()/>
                            <Button on:click=go_back config=ButtonConfig {css_height: sign_in_height.into(), text:"Go Back".into(), css_width: email_input_width.into(), ..Default::default()}/>
                        })
                    },
                    None => Either::Right(view! {
                        <label style:display="none" for="email"></label>
                        <input style:height=sign_in_height maxlength=MAX_EMAIL_SIZE pattern="[^@\\s]+@[^@\\s]+\\.[^@\\s]+" class="sign-in-email-input" node_ref=input_element autocomplete="on" id="email" name="email" placeholder="Enter Email" type="email"/>
                        <Button config=ButtonConfig {button_type: ButtonType::Submit, text: "Sign \u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}".to_string(), css_height: sign_in_height.into(), css_width: email_input_width.into(), class:"sign-in-button".into(), ..Default::default()}/>
                        <SlideToggleCheckbox checkbox_ref=checkbox_element/>
                    })
                }}
            </Show>
        </ActionForm>
    }
}
