use leptos::{either::Either, prelude::*, web_sys::HtmlInputElement};
use crate::{
    components::{button::{Button, ButtonConfig, ButtonType}, 
    message_box::MessageBox, 
    toggle_slider::SlideToggleCheckbox}, 
    utils::{ 
        outcomes::Outcome, 
        proceed, 
        shared_truth::{FULL_LOGO_PATH, MAX_EMAIL_SIZE, USER_CLAIM_AUTH, USER_CLAIM_REFRESH, USER_CLAIM_SIGN_UP}, 
        ui::{Color, Shadow},
        user_types::UserState,
    }
};
use serde::{Deserialize, Serialize};


#[component]
pub fn SignIn() -> impl IntoView {
    let user_resource = expect_context::<Resource<UserState>>();
    let user_state = expect_context::<RwSignal<UserState>>();

    let subject = RwSignal::new(String::new());
    let urgent = RwSignal::new(false);
    let message = RwSignal::new(String::new());

    let name_input_ref = NodeRef::new();
    let email_input_ref = NodeRef::new();

    Effect::new(move || {
        match email_input_ref.get() {
            Some(_) => {
                let element: HtmlInputElement = email_input_ref.get().unwrap();
                element.set_attribute("required", "").ok();
            },
            None => proceed(),
        }
        match name_input_ref.get() {
            Some(_) => {
                let element: HtmlInputElement = name_input_ref.get().unwrap();
                element.set_attribute("required", "").ok();
            },
            None => proceed(),
        }
    });

    let remove_required = move |_| {
        match name_input_ref.get() {
            Some(_) => {
                let element: HtmlInputElement = name_input_ref.get().unwrap();
                element.remove_attribute("required").ok();
            },
            None => proceed(),
        }
    };
    
    let request_email_send = ServerAction::<SendEmail>::new();

    let response = request_email_send.value();

    let sign_in_height = "min(var(--sign-in-element-min-height), var(--sign-in-element-max-height))";
    let shadow_size = "min(calc(5svmax - 5svh), 15px)";
    let email_input_width = "100%";
    let mut box_shadow = Shadow::new(Color::Winter2, "0", "1px", "1px");
    box_shadow.color_intensity = 60;
    box_shadow.spread_radius = "".to_string();
    let box_shadow = box_shadow.css();
    let styles = 
        format!("
        :root {{
            --sign-in-container-padding: calc(6svmax - 6svh);
            --sign-in-element-max-height: 42px;
            --sign-in-element-min-height: 10svmin;
            --sign-in-element-height: {sign_in_height};
            --sign-in-container-gap: calc(1svmax + 2svh);
        }}
        .gone-with-the-wind {{
            position: absolute;
            top: 150%;
        }}
        form {{
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
        }}
        .sign-in-email-input {{
            --input-color: linear-gradient(#fff, #f7f7f7);
            color: {darkslate};
            transition: all 0.3s ease 0s;
            transition: border-width 0.15s ease-out 0s;
            border-color: white;
            font-family: var(--font-family-default);
            font-size: min(var(--sign-in-element-pixel-size), var(--sign-in-element-font-size));
            border: none;
            background: var(--input-color);
            text-decoration: none;
            text-align: center;
            border-radius: 3px;
            height: {sign_in_height};
            width: {email_input_width};
            max-height: var(--sign-in-element-max-height);
            box-shadow: {box_shadow};
        }}
        .sign-in-email-input:hover {{
            text-decoration: none;
            outline: none;
            outline-width: 0;
            border: solid;
            border-width: calc((0.6ch + 0.3svw)/2.2);
            border-color: {frenchgray};
        }}
        .sign-in-email-input:focus {{
            text-decoration: none;
            outline: none;
            outline-width: 0;
            border: solid;
            border-width: calc((0.6ch + 0.3svw)/2.2);
            border-color: {frenchgray};
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
        surround = Shadow::surrounding_shadow(Color::Winter2, shadow_size).css(),
        darkslate = Color::DarkSlate.hex(),
        frenchgray = Color::FrenchGray.hex(),
    );

    let loading_button = move || {
        view! {
            <Button config=ButtonConfig {id: "go_back".into(), button_type: ButtonType::Default, text: "Loading".into(), css_height:sign_in_height.into(), css_width: email_input_width.into(), ..Default::default()}/>
        }
    };

    let continue_button = move || {
        view! {
            <Button config=ButtonConfig {id: "continue".into(), button_type: ButtonType::Link("/"), text: "Continue".into(), css_height:sign_in_height.into(), css_width: email_input_width.into(), ..Default::default()}/>
        }
    };

    let go_back = move |_| {
        response.set(None);
        let new_state = UserState::replace_outcome(user_state.get_untracked(), Outcome::UnresolvedOutcome);
        user_resource.set(Some(new_state.clone()));
        user_state.update_untracked(move |last_user_state| *last_user_state = new_state);
    };

    let display_response = move |outcome: Outcome| {
        match outcome {
            Outcome::UnresolvedOutcome => {
                response.set(None);
            },
            Outcome::UserSignedIn(_) => {
                subject.set("You have been signed in. Continue to the home page.".into());
                urgent.set(false);
                message.set(String::new());
            },
            Outcome::AlreadySignedIn => {
                subject.set("You have been signed in. Continue to the home page.".into());
                urgent.set(false);
                message.set(String::new());
            },
            Outcome::UserNotSignedIn => {
                subject.set("You could not be signed in. Make sure cookies and Javscript/WASM are enabled in your browser.".into());
                urgent.set(true);
                message.set(String::new());
            },
            Outcome::EmailSendSuccess => {
                subject.set("Check your email.".into());
                urgent.set(false);
                message.set(String::new());
            },
            Outcome::EmailSendFailure(_e) => {
                subject.set("Could not send email. Try again in a bit.".into());
                urgent.set(true);
                message.set(String::new());
            },
            Outcome::EmailAlreadyInUse => {
                subject.set("This email could not be used to sign up as it is already in use. Go back and request a sign in email.".into());
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
            Outcome::CreateUserFailure(_) => {
                subject.set("Could not register user. Try again in a bit.".into());
                urgent.set(true);
                message.set(String::new());
            },
            Outcome::TokenExpired => {
                subject.set("Token was expired".into());
                urgent.set(true);
                message.set(String::new());
            },
            Outcome::UserOnlyHasRefreshToken(_) => {
                subject.set(
                    "You were not be signed in. Refreshing your browser might fix this. 
                    If not try enabling Javascript/WASM and cookies in your browser".into()
                );
                urgent.set(true);
                message.set(String::new());
            },
            Outcome::RefreshTokenFailure(_) => {
                let new_state = UserState::replace_outcome(user_state.get_untracked(), Outcome::UnresolvedOutcome);
                user_resource.set(Some(new_state.clone()));
                user_state.update_untracked(move |last_user_state| *last_user_state = new_state);
            },
            any_other_outcome => {
                subject.set(any_other_outcome.to_string());
                urgent.set(true);
                message.set(String::new());
            }
        }
    };

    view! {
        <style>{styles}</style>
        <ActionForm action=request_email_send>
            <img src=FULL_LOGO_PATH alt="LexLinguaLogo" class="sign-in-logo"/>
            <Transition fallback=loading_button>
            <Show when=move || !request_email_send.pending().get() fallback=loading_button>{
                let load_outcome = user_resource.get().unwrap_or_default().sign_in_outcome;
                let action_result = response.get();
                if action_result.is_some() || load_outcome != Outcome::UnresolvedOutcome {
                    let mut outcome = match action_result.unwrap_or(Ok(Outcome::UnresolvedOutcome)) {
                        Ok(outcome) => outcome,
                        Err(e) => Outcome::EmailSendFailure(e.to_string()),
                    };
                    outcome = match load_outcome {
                        Outcome::UnresolvedOutcome => outcome,
                        any_other_outcome => any_other_outcome
                    };
                    display_response(outcome.clone());
                    Either::Left(view! {
                        <MessageBox subject urgent message width=email_input_width.into() only_subject=true top_padding="calc(var(--sign-in-element-height)/2 - 0.5em)".into()/>
                        <Show when=move || !matches!(outcome, Outcome::UserSignedIn(_)) fallback=continue_button>
                        <Button on:click=go_back config=ButtonConfig {id:"goback".into(), css_height: sign_in_height.into(), text:"Go Back".into(), css_width: email_input_width.into(), ..Default::default()}/>
                        </Show>
                    })
                } else {
                    Either::Right(view! {
                        <label class="gone-with-the-wind" for="full_name"></label>
                        <input class="sign-in-email-input gone-with-the-wind" node_ref=name_input_ref style:height=sign_in_height maxlength=MAX_EMAIL_SIZE autocomplete="new-password" id="full_name" tabindex="-60" name="sign_in_form[full_name]" type="name"/>
                        <label class="gone-with-the-wind" for="email"></label>
                        <input style:height=sign_in_height maxlength=MAX_EMAIL_SIZE node_ref=email_input_ref pattern="[^@\\s]+@[^@\\s]+\\.[^@\\s]+" class="sign-in-email-input" autocomplete="on" id="email" name="sign_in_form[email]" placeholder="Enter Email" on:input=remove_required type="email"/>
                        <Button config=ButtonConfig {id: "signin".into(), button_type: ButtonType::Submit, text: "Sign \u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}".to_string(), css_height: sign_in_height.into(), css_width: email_input_width.into(), class:"sign-in-button".into(), ..Default::default()}/>
                        <SlideToggleCheckbox action_form_name="sign_in_form[remember_me]".into()/>
                    })
                }
            }</Show>
            </Transition>
        </ActionForm>
    }
}

#[cfg(feature = "ssr")]
use crate::utils::{
    dynamo_utils::{setup_client, validate_user_and_return_rank},
    back_utils::{build_auth_token, build_sign_up_token, build_refresh_token}, 
    user_types::UserInfo, 
    shared_truth::SIGN_IN_PAGE,
    email_template::{EmailTemplate, EMAIL_FIELD_1, EMAIL_FIELD_1_VALUE, EMAIL_FIELD_2, EMAIL_FIELD_2_VALUE, REDIRECT_LINK},
    shared_utilities::time_till_expiration_pretty,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SignInUpInputs {
    email: String,
    remember_me: Option<String>,
    full_name: String,
}

#[server]
async fn send_email(sign_in_form: SignInUpInputs) -> Result<Outcome, ServerFnError> {
    if !sign_in_form.full_name.is_empty() {
        return Ok(Outcome::VerificationSuccess("You're totally not a bot".to_string()));
    }
    let email_address = &sign_in_form.email;

    let is_trusted = match sign_in_form.remember_me {
        Some(_) => true,
        None => false,
    };

    let client = setup_client().await;

    let outcome = match validate_user_and_return_rank(&client, email_address).await {
        Outcome::PermissionGrantedReturnUser(user) => sign_up_or_in(email_address, false, is_trusted, Some(user)).await,
        Outcome::UserNotFound => sign_up_or_in(email_address, true, is_trusted, None).await,
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
async fn sign_up_or_in(email_address: &str, sign_up: bool, is_trusted: bool, user: Option<UserInfo>) -> Outcome {
    let user = match user {
        Some(user) => user,
        None => UserInfo::default(),
    };
    let api_url = "https://send.api.mailtrap.io/api/send";
    let api_key = std::env::var("MAILTRAP_PASSWORD").unwrap_or_default();

    let (refresh_token, auth_token, sign_up_token) = create_token(email_address, sign_up, is_trusted).await;

    let mut redirect_url = SIGN_IN_PAGE.to_string();
    let mut subject = "Welcome to LexLingua";
    let mut html = EmailTemplate::SignUp.get_template();

    if sign_up {
        redirect_url.push_str(&format!("?{}={}",USER_CLAIM_SIGN_UP, &sign_up_token));
        redirect_url.push_str("&sign-up=true");
        html = html.replace(EMAIL_FIELD_1_VALUE, "Basic").replace(EMAIL_FIELD_1, "Account type");
        html = html.replace(EMAIL_FIELD_2_VALUE, &time_till_expiration_pretty(&sign_up_token)).replace(EMAIL_FIELD_2, "Token expires in");
    } else {
        redirect_url.push_str(&format!("?{}={}&{}={}",USER_CLAIM_REFRESH, &refresh_token, USER_CLAIM_AUTH, &auth_token));
        subject = "Sign in link";
        html = EmailTemplate::SignIn.get_template();
        html = html.replace(EMAIL_FIELD_1_VALUE, &user.lex_rank.to_string()).replace(EMAIL_FIELD_1, "Rank");
        html = html.replace(EMAIL_FIELD_2_VALUE, &time_till_expiration_pretty(&auth_token)).replace(EMAIL_FIELD_2, "Token expires in");
    }

    html = html.replace(REDIRECT_LINK, &redirect_url);

    let email_payload = serde_json::json!({
        "from": {"email": &format!("{}", std::env::var("SENDER_EMAIL").unwrap_or_default()), "name": "LexLingua"},
        "to": [{"email": email_address}],
        "subject": subject,
        "text": redirect_url,
        "html": &html,
    });

    let client = reqwest::Client::new();

    let outcome = match client
    .post(api_url)
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header("Api-Token", api_key)
    .body(email_payload.to_string())
    .send().await {
        Ok(resp) => resp.status().is_success().then(|| Outcome::EmailSendSuccess).unwrap_or(Outcome::EmailSendFailure(resp.text().await.unwrap())),
        Err(e) =>  Outcome::EmailSendFailure(e.to_string()),
    };
    outcome
}
