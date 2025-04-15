use std::collections::HashMap;

#[cfg(not(feature = "ssr"))]
use leptos::logging::debug_warn;
#[cfg(not(feature = "ssr"))]
use web_sys::window;

use super::{database_types::DeckId, date_and_time::{current_time_in_seconds, full_iso_to_secs, Date, PartialDate}, outcomes::Outcome, shared_truth::{EMAIL_CLAIM_KEY, EXP_CLAIM_KEY, PUBLIC_KEY}, sign_in_lib::TokenPair};
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

pub async fn get_cookie_value(name: &str) -> Option<String> {
    #[cfg(not(feature = "ssr"))]
    {
        use leptos::web_sys::wasm_bindgen::JsCast;
        let document = window()?.document()?;
        let html_document = document.dyn_into::<leptos::web_sys::HtmlDocument>().ok()?;
        let cookies = html_document.cookie().ok()?;

        let value = cookies
            .split(';')
            .map(|c| c.trim())
            .find_map(|c| c.strip_prefix(&format!("{}=", name)))
            .map(|s| s.to_string());

        return value;
    }

    #[cfg(feature = "ssr")]
    {
        use leptos_axum::extract;
        use axum_extra::extract::CookieJar;


        let cookie = extract::<CookieJar>().await.ok()?.get(name)?.to_string();
        let (_cookie_name, cookie_value) = cookie.split_once('=').unwrap_or_default();
        Some(cookie_value.into())
    }
}

pub fn set_cookie_value(name: &str, value: &str) -> Result<(), ()> {
    #[cfg(not(feature = "ssr"))]
    {
        use leptos::web_sys::wasm_bindgen::JsCast;
        let Ok(html_document) = leptos::prelude::document().dyn_into::<leptos::web_sys::HtmlDocument>() else {return Err(())};
        _ = html_document.set_cookie(&format!("{name}={value}"));
        return Ok(());
    }

    #[cfg(feature = "ssr")]
    {
        let res = leptos::prelude::expect_context::<leptos_axum::ResponseOptions>();
        let cookie_value = format!("{name}={value}; Path=/; Max-Age=31536000");
        let header_value = axum::http::HeaderValue::from_str(&cookie_value);

        if let Ok(header_value) = header_value {
            res.insert_header(axum::http::header::SET_COOKIE, header_value);
            return Ok(());
        } else {
            return Err(());
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct UserState {
    is_authenticated: bool,
    user: &'static str,
    token: &'static str,
    expiration: &'static str,
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

            return UserState {
                is_authenticated: true,
                user,
                token,
                expiration,
            };
        }
        UserState::default()
    }

    pub fn user(&self) -> &str {
        return self.user;
    }
    pub fn token(&self) -> &str {
        return self.token;
    }
    pub fn expiration(&self) -> &str {
        return self.expiration;
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
