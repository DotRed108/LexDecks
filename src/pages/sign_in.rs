use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use crate::utils_and_structs::{shared_truth::{SIGN_IN_PAGE, USER_CLAIM_AUTH, USER_CLAIM_REFRESH, USER_CLAIM_SIGN_UP}, dynamo_utils::{setup_client, validate_user_standing}, back_utils::{build_auth_token, build_refresh_token, build_sign_up_token}};
#[cfg(feature = "ssr")]
use lettre::{self, message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

use crate::{components::{button::{Button, ButtonConfig, ButtonType}, message_box::MessageBox, toggle_slider::SlideToggleCheckbox}, utils_and_structs::{outcomes::Outcome, shared_truth::{FULL_LOGO_PATH, MAX_EMAIL_SIZE}, ui::{Color, Shadow} }};

#[component]
pub fn SignIn() -> impl IntoView {
    let subject = RwSignal::new(String::new());
    let urgent = RwSignal::new(false);
    let message = RwSignal::new(String::new());

    let input_element = NodeRef::new();
    let checkbox_element = NodeRef::new();

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
            Outcome::EmailSendFailure(_e) => {
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
            Outcome::UserSuspended(_date) => {
                subject.set("This email is associated with a suspended account.".into());
                urgent.set(true);
                message.set(String::new());
            },
            any_other_outcome => {
                subject.set(any_other_outcome.to_string());
                urgent.set(true);
                message.set(String::new());
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
                        <input style:height=sign_in_height maxlength=MAX_EMAIL_SIZE pattern="[^@\\s]+@[^@\\s]+\\.[^@\\s]+" class="sign-in-email-input" node_ref=input_element autocomplete="on" id="email" name="sign_in_form[email]" placeholder="Enter Email" type="email"/>
                        <Button config=ButtonConfig {button_type: ButtonType::Submit, text: "Sign \u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}".to_string(), css_height: sign_in_height.into(), css_width: email_input_width.into(), class:"sign-in-button".into(), ..Default::default()}/>
                        <SlideToggleCheckbox action_form_name="sign_in_form[remember_me]".into() checkbox_ref=checkbox_element/>
                    })
                }}
            </Show>
        </ActionForm>
    }
}

const SENDER_EMAIL: &str = "LexLingua <mailtrap@demomailtrap.com>";
const DEMO_MAILTRAP_PASSWORD: &str = "b8bd86f42193f60cc1ff4420ffb68a59";
const MAILTRAP_USERNAME: &str = "api";
const _MAILTRAP_PASSWORD: &str = "a0b103db49012c23dfd54092e886509b";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SignInUpInputs {
    email: String,
    remember_me: Option<String>,
}

#[server]
async fn send_email(sign_in_form: SignInUpInputs) -> Result<Outcome, ServerFnError> {
    let email_address = &sign_in_form.email;

    let is_trusted = match sign_in_form.remember_me {
        Some(_) => true,
        None => false,
    };

    let client = setup_client().await;

    let outcome = match validate_user_standing(&client, email_address).await {
        Outcome::PermissionGranted(_) => sign_up_or_in(email_address, false, is_trusted).await,
        Outcome::UserNotFound => sign_up_or_in(email_address, true, is_trusted).await,
        any_other_outcome => return Ok(any_other_outcome)
    };

    Ok(outcome)
}

#[cfg(feature = "ssr")]
async fn create_token(email_address: &str, sign_up: bool, is_trusted: bool) -> (String, String, String) {
    // refresh_token, auth_token, sign_up_token
    if sign_up {
        let sign_up_token = build_sign_up_token(is_trusted, email_address).unwrap();
        return ("".to_owned(), "".to_owned(), sign_up_token)
    } else {
        let refresh_token = build_refresh_token(is_trusted, email_address).unwrap();
        let auth_token = build_auth_token(is_trusted, email_address).unwrap();
        return (refresh_token, auth_token, "".to_string());
    }
}

#[cfg(feature = "ssr")]
async fn sign_up_or_in(email_address: &str, sign_up: bool, is_trusted: bool) -> Outcome {
    let recipient = format!("LexLingua User <{email_address}>");
    
    let message: Message;
    let message_bldr = Message::builder()
    .from(SENDER_EMAIL.parse().unwrap())
    .reply_to(SENDER_EMAIL.parse().unwrap())
    .to(recipient.parse().unwrap());

    let (refresh_token, auth_token, sign_up_token) = create_token(email_address, sign_up, is_trusted).await;

    let mut redirect_url = SIGN_IN_PAGE.to_string();

    if sign_up {
        redirect_url.push_str(&format!("?{}={}",USER_CLAIM_SIGN_UP, &sign_up_token));
        redirect_url.push_str("&sign-up=true");
        let Ok(msg) = message_bldr
        .subject("Welcome to LexLingua")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(redirect_url)) else {
            return Outcome::EmailSendFailure("email could not be built".to_string());
        };
        message = msg;
    } else {
        redirect_url.push_str(&format!("?{}={}&{}={}",USER_CLAIM_REFRESH, &refresh_token, USER_CLAIM_AUTH, &auth_token));
        let Ok(msg) = message_bldr
        .subject("Sign in link")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(redirect_url)) else {
            return Outcome::EmailSendFailure("email could not be built".to_string());
        };
        message = msg;
    }

    let creds = Credentials::new(MAILTRAP_USERNAME.to_string(), DEMO_MAILTRAP_PASSWORD.to_string());

    let Ok(smtp_bldr) = SmtpTransport::relay("live.smtp.mailtrap.io") else {return Outcome::EmailSendFailure("Could not connect to smtp relay".to_string());};
    let mailer = smtp_bldr.credentials(creds).build();

    match mailer.send(&message) {
        Ok(response) => {
            if response.is_positive() {
                Outcome::EmailSendSuccess
            } else {
                Outcome::EmailSendFailure("Email recieved response but failed".to_string())
            }
        },
        Err(e) => Outcome::EmailSendFailure(format!("Email failed to send with error {e}")),
    }
}
