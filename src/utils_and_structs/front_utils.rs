use std::collections::HashMap;

use crate::utils_and_structs::date_and_time::full_iso_to_secs;

use super::database_types::{DeckId, DeckList};
use super::date_and_time::{current_time_in_seconds, Date, PartialDate};
use super::outcomes::Outcome;
use super::queries::ValidQueryTypes;
use super::shared_truth::{
    MAX_LEVELS, PUBLIC_KEY
};
use super::sign_in_lib::TokenPair;
use leptos::logging::debug_warn;
use leptos::{web_sys, web_sys::window};
use pasetors::errors::ClaimValidationError;
use pasetors::{
    errors::Error,
    keys::AsymmetricPublicKey,
    token::{TrustedToken, UntrustedToken},
    version4::{self, V4},
    Public,
};
use web_sys::{Element, HtmlImageElement};

pub const EXP_CLAIM_KEY: &str = "exp";
pub const EMAIL_CLAIM_KEY: &str = "user";

pub const S3_CREATION_DATE_URL_PARAM: &str = "X-Amz-Date=";
pub const S3_EXPIRATION_URL_PARAM: &str = "X-Amz-Expires=";

pub fn get_item_from_local_storage(key: &str) -> Option<String> {
    if cfg!(feature="ssr") {
        return None;
    }

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
    Some(item)
}

pub fn store_item_in_local_storage(key: &str, value: &str) -> Result<(), ()> {
    if cfg!(feature="ssr") {
        return Err(());
    }

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

pub fn clear_element_classes_and_add_new(element: Element, class: String) {
    let classes = element.class_name();
    if !classes.is_empty() {
        let classes_as_strs = classes.split(" ");
        for class_str in classes_as_strs {
            let _ = element.class_list().remove_1(class_str);
        }
    }
    let _ = element.class_list().add_1(&class);
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

    // if is_expired(&expiration, None) {
    //     return Err(Error::ClaimValidation(ClaimValidationError::Exp));
    // }

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

pub fn expiration_date_from_s3_url(url: &str) -> Option<u64> {
    /************* S3 URL EXAMPLE ***************/
    // https://lexpfpbucket.s3.us-east-2.amazonaws.com/default1.avif
    // ?x-id=GetObject&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=
    // AKIAZI2LJCXORLLPSQWQ%2F20240509%2Fus-east-2%2Fs3%2Faws4_request&
    // X-Amz-Date=20240509T053932Z&X-Amz-Expires=20&X-Amz-SignedHeaders=host&
    // X-Amz-Signature=8b62b651a8600f7c4fa3568523e490e1016fb03e11ea42068483e3a9d66be4db
    /************ END EXAMPLE ******************/
    let Some(creation_date_index) = url.find(S3_CREATION_DATE_URL_PARAM) else {
        debug_warn!("no creation date found in url");
        return None;
    };
    let (_, midget_url) = url.split_at(creation_date_index + S3_CREATION_DATE_URL_PARAM.len());

    let Some(end_of_creation_date_index) = midget_url.find('&') else {
        debug_warn!("could not find end of creation date in url");
        return None;
    };
    let (creation_date, _) = midget_url.split_at(end_of_creation_date_index);

    let Some(expiration_index) = url.find(S3_EXPIRATION_URL_PARAM) else {
        debug_warn!("no expiration found in url");
        return None;
    };
    let (_, midget_url) = url.split_at(expiration_index + S3_EXPIRATION_URL_PARAM.len());

    let Some(end_of_expiration_index) = midget_url.find('&') else {
        debug_warn!("could not find end of expiration in url");
        return None;
    };
    let (expiration, _) = midget_url.split_at(end_of_expiration_index);

    let formatted_date = format!(
        "{}-{}-{}T{}:{}:{}Z",
        &creation_date[..4],
        &creation_date[4..6],
        &creation_date[6..8],
        &creation_date[9..11],
        &creation_date[11..13],
        &creation_date[13..15]
    );

    let creation_date = web_sys::js_sys::Date::parse(&formatted_date);

    if creation_date == 0.0 {
        debug_warn!("could not parse creation date string to javascript date object");
        return None;
    };

    let expiration: f64 = match expiration.parse() {
        Ok(num) => num,
        Err(_) => {
            debug_warn!("could not parse expiration string as number");
            return None;
        }
    };
    let expiration_date = (creation_date / 1000.0) + expiration;

    Some(expiration_date as u64) // time from unix epoch in seconds
}

pub fn s3_url_expired(url: &str) -> bool {
    let Some(expiration_date) = expiration_date_from_s3_url(url) else {
        return true;
    };

    let current_time = current_time_in_seconds();

    current_time > expiration_date
}

pub fn image_cached(url: &str) -> bool {
    let image = match HtmlImageElement::new() {
        Ok(image) => image,
        Err(_) => {
            debug_warn!("could not test url");
            return false;
        }
    };

    image.set_src(url);

    let is_cached = image.complete() || image.natural_width() > 0;

    if !!!is_cached {
        debug_warn!("image is not cached")
    }

    is_cached
}

pub fn frontend_query_validation(query: &ValidQueryTypes, valid_decks: DeckList) -> Outcome {    
    match query {
        ValidQueryTypes::NotesByLevel(deck_id, levels) => {
            debug_warn!("current deck id {}", deck_id.to_string());
            if !!!valid_decks.contains(&deck_id) {return Outcome::UserDoesNotHavePermission};

            for level in levels {
                if level > &MAX_LEVELS {return Outcome::InvalidRequest}
            }
        },
        ValidQueryTypes::NotesById(deck_id, _) => {
            if !!!valid_decks.contains(&deck_id) {return Outcome::UserDoesNotHavePermission};
        },
        ValidQueryTypes::NotesByType(deck_id, _) => {
            if !!!valid_decks.contains(&deck_id) {return Outcome::UserDoesNotHavePermission};
        }
        _ => return Outcome::InvalidRequest,
    }
    Outcome::PermissionGranted("Query Likely Valid".to_string())
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
