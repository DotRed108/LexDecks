use std::{borrow::Cow, collections::HashMap, future::Future};

#[cfg(not(feature = "ssr"))]
use leptos::logging::debug_warn;
use leptos::prelude::{server, GetUntracked, RwSignal, ServerFnError, Set};
use leptos_router::hooks::use_query_map;
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "ssr"))]
use web_sys::window;


use super::{database_types::DeckId, date_and_time::{current_time_in_seconds, full_iso_to_secs, Date, PartialDate}, outcomes::Outcome, shared_truth::{EMAIL_CLAIM_KEY, EXP_CLAIM_KEY, IS_TRUSTED_CLAIM, LOCAL_AUTH_TOKEN_KEY, LOCAL_REFRESH_TOKEN_KEY, PUBLIC_KEY, USER_CLAIM_AUTH, USER_CLAIM_REFRESH}, sign_in_lib::TokenPair};
use pasetors::{errors::{ClaimValidationError, Error}, keys::AsymmetricPublicKey, token::{TrustedToken, UntrustedToken}, version4::{self, V4}, Public};

#[allow(unused)]
pub fn get_item_from_local_storage(key: &str) -> Option<String> {
    #[cfg(feature = "ssr")] {
        return None;
    }

    #[cfg(not(feature = "ssr"))] {
        let item = match web_sys::window() {
            Some(window) => match window.local_storage() {
                Ok(t) => match t {
                    Some(local_storage) => match local_storage.get(key) {
                        Ok(item) => match item {
                            Some(value) => value,
                            None => {
                                debug_warn!("value from key: {key} not found in local storage");
                                return None;
                            }
                        },
                        Err(_) => {
                            debug_warn!("local storage key: {key} not found");
                            return None;
                        }
                    },
                    None => {
                        debug_warn!("local storage not found");
                        return None;
                    }
                },
                Err(_) => {
                    debug_warn!("local storage not found");
                    return None;
                }
            },
            None => {
                debug_warn!("window not found while getting item from local storage");
                return None;
            }
        };
        return Some(item)
    }
}

#[allow(unused)]
pub fn store_item_in_local_storage(key: &str, value: &str) -> Result<(), ()> {
    #[cfg(feature = "ssr")] {
        return Err(());
    }

    #[cfg(not(feature = "ssr"))] {
        let local_storage = match window().unwrap().local_storage() {
            Ok(possible_storage) => match possible_storage {
                Some(storage) => storage,
                None => {
                    debug_warn!("no storage found");
                    return Err(());
                }
            },
            Err(_) => {
                debug_warn!("could not retrieve storage");
                return Err(());
            }
        };
        match local_storage.set_item(key, value) {
            Ok(_) => (),
            Err(_) => {
                debug_warn!("could not store value in key: {}", key);
                return Err(());
            }
        };
        let event = match leptos::ev::Event::new(&format!("storage-{key}")) {
            Ok(event) => event,
            Err(_) => {
                debug_warn!("value stored in key: {key}, but event not created");
                return Err(());
            }
        };

        match window().unwrap().dispatch_event(&event) {
            Ok(_) => (),
            Err(_) => {
                debug_warn!("value stored in key: {key}, but event not dispatched");
                return Err(());
            }
        };

        Ok(())
    }
}

pub fn verify_token(token: &str) -> Result<TrustedToken, Error> {
    let public_key = AsymmetricPublicKey::<V4>::from(&PUBLIC_KEY)?;

    let untrusted_token = UntrustedToken::<Public, V4>::try_from(token)?;

    let trusted_token = version4::PublicToken::verify(
        &public_key,
        &untrusted_token,
        None,
        Some(b"implicit assertion"),
    )?;

    let expiration = match get_claim(&trusted_token, EXP_CLAIM_KEY) {
        Some(exp) => exp,
        None => return Err(Error::ClaimValidation(ClaimValidationError::NoExp)),
    };

    if is_expired(&expiration, None) {
        return Err(Error::ClaimValidation(ClaimValidationError::Exp));
    }

    Ok(trusted_token)
}

pub fn verify_then_return_outcome(token: &str) -> Outcome {
    let outcome = match verify_token(token) {
        Ok(_) => Outcome::VerificationSuccess(token.into()),
        Err(e) => match e {
            Error::ClaimValidation(claim_validation_error) => match claim_validation_error {
                ClaimValidationError::Exp => Outcome::TokenExpired,
                _any_other_error => Outcome::VerificationFailure
            },
            _any_other_error => Outcome::VerificationFailure,
        },
    };
    outcome
}

pub fn verify_token_pair(token_pair: &TokenPair) -> Result<(TrustedToken, TrustedToken), Error> {
    let refresh_token = token_pair.get_refresh_token();
    let auth_token = token_pair.get_auth_token();
    Ok((verify_token(&refresh_token)?, verify_token(&auth_token)?))
}

pub fn is_expired(expiration_date: &str, offset_in_seconds: Option<u64>) -> bool {
    let offset = offset_in_seconds.unwrap_or(0);
    let Some(expiration_time) = full_iso_to_secs(expiration_date) else {return true};
    let expiration_time = expiration_time - offset;

    let current_time = current_time_in_seconds();

    if expiration_time > current_time {
        false
    } else {
        true
    }
}

pub fn get_claim(token: &TrustedToken, claim_key: &str) -> Option<String> {
    let payload = token.payload();

    let claim_key = format!("\"{claim_key}\":\"");

    let Some((_, claim)) = payload.split_once(&claim_key) else {return None};
    let (claim, _) = claim.split_once("\"").expect("There should be a quotation mark at the end of the value");

    Some(claim.to_string())
}

pub fn expiration_in_secs(token: &str) -> u64 {
    let Ok(trusted_token) = verify_token(&token) else {return 0};
    let Some(expiration) = get_claim(&trusted_token, EXP_CLAIM_KEY) else {return 0};

    full_iso_to_secs(&expiration).unwrap_or_default()
}

pub fn time_till_expiration_in_seconds(token: &str) -> u64 {
    expiration_in_secs(token) - current_time_in_seconds()
}

pub fn time_till_expiration_pretty(token: &str) -> String {
    let seconds_till = time_till_expiration_in_seconds(token);
    if seconds_till > Date::SECONDS_IN_DAY {
        let days = seconds_till/Date::SECONDS_IN_DAY;
        let mut plural = "";
        if days > 1 {
            plural = "s";
        }
        return format!("{days} Day{plural}");
    } else if seconds_till > Date::SECONDS_IN_HOUR {
        let hours = seconds_till/Date::SECONDS_IN_HOUR;
        let mut plural = "";
        if hours > 1 {
            plural = "s";
        }
        return format!("{hours} Hour{plural}");
    } else if seconds_till > Date::SECONDS_IN_MINUTE {
        let minutes = seconds_till/Date::SECONDS_IN_MINUTE;
        let mut plural = "";
        if minutes > 1 {
            plural = "s";
        }
        return format!("{minutes} Minute{plural}");
    }
    return "Unknown".to_string();
}

pub fn set_token_cookie(token: &str) -> Result<(), ()> {
    let Ok(trusted_token) = verify_token(token) else {return Err(())};
    let name = match get_claim(&trusted_token, USER_CLAIM_REFRESH) {
        Some(_) => LOCAL_REFRESH_TOKEN_KEY,
        None => match get_claim(&trusted_token, USER_CLAIM_AUTH) {
            Some(_) => LOCAL_AUTH_TOKEN_KEY,
            None => return Err(()),
        }
    };
    let expiration = time_till_expiration_in_seconds(token);
    
    set_cookie_value(name, token, expiration)
}

pub fn set_cookie_value(name: &str, value: &str, expiration: u64) -> Result<(), ()> {
    let cookie_string = format!("{name}={value}; Path=/; Max-age={expiration}; Secure=true; SameSite=Strict;");
    #[cfg(not(feature = "ssr"))]
    {
        use leptos::web_sys::wasm_bindgen::JsCast;
        let Ok(html_document) = leptos::prelude::document().dyn_into::<leptos::web_sys::HtmlDocument>() else {return Err(())};
        _ = html_document.set_cookie(&cookie_string);
        return Ok(());
    }

    #[cfg(feature = "ssr")]
    {
        let res = leptos::prelude::expect_context::<leptos_axum::ResponseOptions>();
        let header_value = axum::http::HeaderValue::from_str(&cookie_string);

        if let Ok(header_value) = header_value {
            res.append_header(axum::http::header::SET_COOKIE, header_value);
            return Ok(());
        } else {
            return Err(());
        }
    }
}

pub async fn get_cookie_value(name: &str) -> Option<String> {
    #[cfg(not(feature = "ssr"))]
    return super::front_utils::get_cookie_value_client(name);

    #[cfg(feature = "ssr")]
    {
        use leptos_axum::extract;
        use tower_cookies::Cookies;

        let cookies = extract::<Cookies>().await.ok()?;
        let cookie = cookies.get(name);
        let Some(cookie) = cookie else {return None};
        Some(cookie.value_trimmed().to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UserState {
    is_authenticated: bool,
    user: Cow<'static, str>,
    token: Cow<'static, str>,
    expiration: Cow<'static, str>,
}

impl UserState {
    pub fn from_token_or_default(stored_auth_token: &String) -> Self {
        let auth_token = stored_auth_token;
        if !auth_token.is_empty() {
            let token = match verify_token(&auth_token) {
                Ok(trusted_token) => trusted_token,
                Err(_) => return UserState::default(),
            };

            let expiration = match get_claim(&token, EXP_CLAIM_KEY) {
                Some(expiration_date) => expiration_date,
                None => return UserState::default(),
            };

            if is_expired(&expiration, Some(18000)) {
                return UserState::default();
            }

            let user = match get_claim(&token, EMAIL_CLAIM_KEY) {
                Some(user) => user,
                None => return UserState::default(),
            };

            let user: &'static str = String::leak(user);
            let expiration: &'static str = String::leak(expiration);
            let token: &'static str = String::leak(auth_token.to_owned());

            store_item_in_local_storage(LOCAL_AUTH_TOKEN_KEY, token).unwrap_or_default();
            set_token_cookie(token).unwrap_or_default();
            return UserState {
                is_authenticated: true,
                user: Cow::Borrowed(user),
                token: Cow::Borrowed(token),
                expiration: Cow::Borrowed(expiration),
            };
        }
        UserState::default()
    }

    pub async fn find_token_or_default() -> Self {
        // check cookie
        let cookie_auth_token = get_cookie_value(LOCAL_AUTH_TOKEN_KEY).await.unwrap_or_default();
        let user_state = UserState::from_token_or_default(&cookie_auth_token);
        if !(user_state == UserState::default()) {
            return user_state
        }
        // check local storage
        let local_storage_auth_token = get_item_from_local_storage(LOCAL_AUTH_TOKEN_KEY).unwrap_or_default();
        let user_state = UserState::from_token_or_default(&local_storage_auth_token);
        if !(user_state == UserState::default()) {
            return user_state
        }
        // check for refresh token in cookie then use it to get an auth token
        let cookie_refresh_token = get_cookie_value(LOCAL_REFRESH_TOKEN_KEY).await.unwrap_or_default();
        let cookie_refresh_outcome = match verify_then_return_outcome(&cookie_refresh_token) {
            Outcome::VerificationSuccess(token) => use_refresh_token(token).await.unwrap_or_default(),
            any_other_outcome => any_other_outcome,
        };
        let user_state = match cookie_refresh_outcome {
            Outcome::TokensRefreshed(tokens) => UserState::from_token_or_default(&tokens.get_auth_token()),
            _any_other_outcome => UserState::default(),
        };
        if !(user_state == UserState::default()) {
            return user_state
        }
        // check for refresh token in local storage then use it to get an auth token
        let local_storage_refresh_token = get_item_from_local_storage(LOCAL_REFRESH_TOKEN_KEY).unwrap_or_default();
        let local_storage_refresh_token = match verify_then_return_outcome(&local_storage_refresh_token) {
            Outcome::VerificationSuccess(token) => use_refresh_token(token).await.unwrap_or_default(),
            any_other_outcome => any_other_outcome,
        };
        let user_state = match local_storage_refresh_token {
            Outcome::TokensRefreshed(tokens) => UserState::from_token_or_default(&tokens.get_auth_token()),
            _any_other_outcome => UserState::default(),
        };
        if !(user_state == UserState::default()) {
            return user_state
        }
        return UserState::default()
    }

    pub async fn from_token_pair(token_pair: &TokenPair) -> Self {
        let user_state = UserState::from_token_or_default(&token_pair.get_auth_token());
        if user_state != UserState::default() {
            store_item_in_local_storage(LOCAL_REFRESH_TOKEN_KEY, &token_pair.get_refresh_token()).unwrap_or_default();
            set_token_cookie(&token_pair.get_refresh_token()).unwrap_or_default();
        }
        user_state
    }

    pub fn user(&self) -> &str {
        return &self.user;
    }
    pub fn token(&self) -> &str {
        return &self.token;
    }
    pub fn expiration(&self) -> &str {
        return &self.expiration;
    }
    pub fn is_authenticated(&self) -> bool {
        return self.is_authenticated;
    }
}

pub fn get_fake_review_schedule(_deck_id: DeckId) -> (HashMap<PartialDate, usize>, usize) {
    let mut fake_review_schedule = HashMap::with_capacity(Date::JAN.days(1970) * 3);

    fake_review_schedule.insert(PartialDate::day_and_month(2, Date::MAR), 10);
    fake_review_schedule.insert(PartialDate::day_and_month(30, Date::MAR), 3);
    fake_review_schedule.insert(PartialDate::day_and_month(1, Date::APR), 7);
    fake_review_schedule.insert(PartialDate::day_and_month(16, Date::APR), 37);
    fake_review_schedule.insert(PartialDate::day_and_month(17, Date::APR), 21);
    fake_review_schedule.insert(PartialDate::day_and_month(6, Date::MAY), 5);

    let highest_review_amount = 37;

    return (fake_review_schedule, highest_review_amount)
}

pub fn update_signal_with_future<T, F>(signal: RwSignal<T>, future: F)
where
    T: 'static + Clone + Send + Sync,
    F: Future<Output = T> + 'static + Send,
{   
    #[cfg(feature = "ssr")]
    tokio::task::block_in_place(|| {
        signal.set(tokio::runtime::Handle::current().block_on(future))
    });
    #[cfg(not(feature = "ssr"))]
    leptos::task::spawn_local(async move {
        let result = future.await;
        signal.set(result);
    });
}

pub fn get_url_query(key: &str) -> Option<String> {
    let url_queries = use_query_map().get_untracked();

    let query = match url_queries.get(key) {
        Some(query) => query,
        None => return None,
    };

    Some(query)
}

#[server]
pub async fn use_refresh_token(refresh_token: String) -> Result<Outcome, ServerFnError> {
    #[cfg(feature="ssr")]
    use crate::utils::{back_utils::generate_auth_token, dynamo_utils::{setup_client, validate_user_standing}};
    let Ok(trusted_token) = verify_token(&refresh_token) else {return Ok(Outcome::VerificationFailure)};

    let Some(email) = get_claim(&trusted_token, USER_CLAIM_REFRESH) else {return Ok(Outcome::VerificationFailure)};

    let trusted_device = match get_claim(&trusted_token, IS_TRUSTED_CLAIM) {
        Some(claim) => claim.parse().unwrap_or(false),
        None => false,
    };
    
    let client = setup_client().await;

    let outcome = match validate_user_standing(&client, &email).await {
        Outcome::PermissionGranted(_) => generate_auth_token(&email, &refresh_token, trusted_device),
        any_other_outcome => return Ok(any_other_outcome),
    };

    Ok(outcome)
}

pub fn initial_user_state() -> UserState {
    #[cfg(feature="ssr")]
    return UserState::from_token_or_default(&tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(get_cookie_value(LOCAL_AUTH_TOKEN_KEY)).unwrap_or_default()
    }));
    #[cfg(not(feature="ssr"))]
    return UserState::from_token_or_default(&super::front_utils::get_cookie_value_client(LOCAL_AUTH_TOKEN_KEY).unwrap_or_default());  
}
