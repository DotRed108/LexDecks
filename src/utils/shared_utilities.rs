use std::{collections::HashMap, future::Future, str::FromStr};

#[cfg(not(feature = "ssr"))]
use leptos::logging::debug_warn;
#[cfg(not(feature = "ssr"))]
use web_sys::window;
#[cfg(not(feature = "ssr"))]
use crate::utils::cache::clear_cache;

use crate::utils::{
    database_types::DeckId,
    date_and_time::{current_time_in_seconds, full_iso_to_secs, Date, PartialDate},
    outcomes::Outcome, 
    shared_truth::{EXP_CLAIM_KEY, LOCAL_AUTH_TOKEN_KEY, LOCAL_REFRESH_TOKEN_KEY, PUBLIC_KEY, USER_CLAIM_AUTH, USER_CLAIM_REFRESH}, 
    sign_in_lib::TokenPair,
};

use leptos::prelude::{RwSignal, Set};
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
    let cookie_string = format!("{name}={value}; Path=/; Max-age={expiration}; Secure=true; SameSite=Lax;");
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

pub fn clear_cookie(name: &str) -> Result<(), ()> {
    let cookie_string = format!("{name}=\"\"; Path=/; Max-age=-9999; Secure=true; SameSite=Lax; expires=Thu, 01 Jan 1970 00:00:01 GMT;");
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

pub fn clear_user_cache_and_cookies() {
    #[cfg(not(feature="ssr"))]
    {
        let _ = clear_cache(super::shared_truth::LOCAL_USER_INFO_KEY);
        let _ = clear_cache(LOCAL_AUTH_TOKEN_KEY);
        let _ = clear_cache(LOCAL_REFRESH_TOKEN_KEY);
    }
    let _ = clear_cookie(LOCAL_AUTH_TOKEN_KEY);
    let _ = clear_cookie(LOCAL_REFRESH_TOKEN_KEY);
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

pub async fn get_url_query(query_key: &str) -> Option<String> {
    #[cfg(not(feature="ssr"))]
    let url = window().unwrap().location().href().unwrap_or_default();
    #[cfg(feature="ssr")]
    let url: axum::http::Uri = leptos_axum::extract().await.unwrap_or_default();
    #[cfg(feature="ssr")]
    let url = url.to_string();

    let url = url::Url::from_str(&url).unwrap();

    for (key, value) in url.query_pairs() {
        if query_key == key {
            return Some(value.to_string())
        }
    }
    None
}

#[allow(unused_variables)]
pub fn get_url_query_client(query_key: &str) -> Option<String> {
    #[cfg(not(feature="ssr"))]
    {
        let url = window().unwrap().location().href().unwrap_or_default();

        let url = url::Url::from_str(&url).unwrap();

        for (key, value) in url.query_pairs() {
            if query_key == key {
                return Some(value.to_string())
            }
        }
    }
    None
}

pub fn excluded_from_auth(url: String) -> bool {
    let excluded_server_functions = ["send_email", "use_refresh_token", "create_user"];

    for function_name in excluded_server_functions {
        if url.contains(function_name) {
            return true;
        }
    }
    false
}
