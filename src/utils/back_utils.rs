use std::{str::FromStr, time::{Duration, SystemTime, UNIX_EPOCH}};

use crate::utils::database_types::{Asset, S3Address};
use crate::utils::user_types::Standing;
use axum::http::{HeaderMap, Method};
use pasetors::{claims::{Claims, ClaimsValidationRules}, errors::Error as PasetoError, keys::{AsymmetricPublicKey, AsymmetricSecretKey}, public, token::{TrustedToken, UntrustedToken}, version4::V4, Public};
use serde::{Deserialize, Serialize};
use leptos_axum::extract;

use super::{date_and_time::current_time_in_seconds, outcomes::Outcome, shared_truth::{IS_TRUSTED_CLAIM, PUBLIC_KEY, USER_CLAIM_AUTH, USER_CLAIM_REFRESH, USER_CLAIM_SIGN_UP}, sign_in_lib::TokenPair};

pub const PUBLIC_DECKS_TABLE: &str = "LEXDecks";

pub const PUBLIC_DECKS_BUCKET: &str = "lexpublicdecksbucket";

pub const PUBLIC_DECKS_STAGING_BUCKET: &str = "lexpublicdecksstagingbucket";

pub const PFP_BUCKET: &str = "lexpfpbucket";

pub const USERS_TABLE: &str = "LEXUsers";

pub const UPLOAD_TOKEN_PRICE_IN_DOLLARS: f64 = 0.20;

#[derive(Serialize, Deserialize)]
pub struct PasetoPrivateKey(#[serde(with = "serde_arrays")] [u8; 64]);

impl PasetoPrivateKey {
    pub fn from_key(key: AsymmetricSecretKey<V4>) -> PasetoPrivateKey {
        let bytes = key.as_bytes();
        let mut byte_array = [0; 64];
        for (index, byte) in bytes.iter().enumerate() {
            byte_array[index] = byte.to_owned();
        }
        PasetoPrivateKey(byte_array)
    }
    pub fn get_key() -> [u8; 64] {
        PasetoPrivateKey::from_str(&std::env::var("PASETO_PRIVATE_KEY").unwrap()).unwrap().0
    }
}

impl ToString for PasetoPrivateKey {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for PasetoPrivateKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let private_key = match serde_json::from_str(s) {
            Ok(info) => info,
            Err(_) => return Err(()),
        };
        Ok(private_key)
    }
}

pub fn is_in_good_standing(standing: &String) -> bool {
    let standing = Standing::from_str(standing).expect("wut");
    let current_date = current_time_in_seconds();

    match standing {
        Standing::WUser => true,
        Standing::Suspended(suspension_date) => if suspension_date < current_date {
            return true;
        } else {
            return false;
        },
    }
}

pub fn return_suspension_date_if_suspended(standing: &String) -> Option<u64> {
    if is_in_good_standing(standing) {
        return None;
    } else {
        if let Standing::Suspended(date) = Standing::from_str(standing).expect("wut duh hec") {
            return Some(date);
        } else {
            return None;
        }
    }
}

pub fn is_in_active_decks(active_deck_list: &Vec<String>, deck: &String) -> bool {
    if active_deck_list.contains(deck) {
        return true;
    } else {
        return false;
    }
}

pub fn get_default_pfp() -> Asset {
    let defaults = [1, 2];

    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_nanos();

    let nanos_as_chars = nanos.to_string();
    let mut nanos_as_chars = nanos_as_chars.chars();

    let first_num = nanos_as_chars.next_back().unwrap_or('0');
    let second_num = nanos_as_chars.next_back().unwrap_or('0');

    let mut first_num: i32 = first_num.to_string().parse().unwrap_or_default();
    let second_num: i32 = second_num.to_string().parse().unwrap_or_default();

    if first_num > 7 {
        first_num = 10
    } else {
        first_num = 0
    }

    let final_num = first_num + second_num;

    let mut last_distance_from: i32 = 250;
    let mut current_selection = 250;
    for default_num in defaults {
        let distance_from = final_num - default_num;
        if distance_from.abs() < last_distance_from.abs() {
            current_selection = default_num;
            last_distance_from = distance_from;
        }
    }
    Asset::PFP(S3Address {bucket: PFP_BUCKET.to_owned(), key: format!("default{current_selection}.avif")})
}

pub fn choose_table() -> String {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_nanos();

    let nanos_as_chars = nanos.to_string();
    let mut nanos_as_chars = nanos_as_chars.chars();

    let random_number = nanos_as_chars.next_back().unwrap_or('0');

    format!("{PUBLIC_DECKS_TABLE}{random_number}")
}

pub fn verify_token(token: &str) -> Result<TrustedToken, PasetoError> {
    let public_key = AsymmetricPublicKey::<V4>::from(&PUBLIC_KEY)?;
    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Public, V4>::try_from(token)?;
    let trusted_token = public::verify(&public_key, &untrusted_token, &validation_rules, None, Some(b"implicit assertion"))?;

    Ok(trusted_token)
}

pub fn build_refresh_token(is_trusted: bool, email_address: &str) -> Result<String, PasetoError> {
    let one_hour = 3600;
    let one_day = one_hour * 24;
    let one_year = 31536000;

    let mut claims;
    if is_trusted {
        claims = Claims::new_expires_in(&Duration::from_secs(one_year))?;
    } else {
        claims = Claims::new_expires_in(&Duration::from_secs(one_day))?;
    }
    claims.add_additional(USER_CLAIM_REFRESH, email_address)?;
    claims.add_additional(IS_TRUSTED_CLAIM, is_trusted.to_string())?;

    let private_key = AsymmetricSecretKey::<V4>::from(&PasetoPrivateKey::get_key())?;
    let refresh_token = public::sign(&private_key, &claims, None, Some(b"implicit assertion"))?;

    Ok(refresh_token)
}

pub fn build_sign_up_token(is_trusted: bool, email_address: &str) -> Result<String, PasetoError> {
    let one_hour = 3600;

    let mut claims = Claims::new_expires_in(&Duration::from_secs(one_hour))?;
    claims.add_additional(USER_CLAIM_SIGN_UP, email_address)?;
    claims.add_additional(IS_TRUSTED_CLAIM, is_trusted.to_string())?;

    let private_key = AsymmetricSecretKey::<V4>::from(&PasetoPrivateKey::get_key())?;
    let sign_up_token = public::sign(&private_key, &claims, None, Some(b"implicit assertion"))?;

    Ok(sign_up_token)
}

pub fn build_auth_token(is_trusted: bool, email_address: &str) -> Result<String, PasetoError> {
    let one_hour = 3600;
    let one_day = one_hour * 24;
    let one_year = 31536000;
    let one_month = one_year / 12;

    let mut claims;
    if is_trusted {
        claims = Claims::new_expires_in(&Duration::from_secs(one_month))?;
    } else {
        claims = Claims::new_expires_in(&Duration::from_secs(one_day))?;
    }
    claims.add_additional(USER_CLAIM_AUTH, email_address)?;
    claims.add_additional(IS_TRUSTED_CLAIM, is_trusted.to_string())?;

    let private_key = AsymmetricSecretKey::<V4>::from(&PasetoPrivateKey::get_key())?;
    let auth_token = public::sign(&private_key, &claims, None, Some(b"implicit assertion"))?;

    Ok(auth_token)
}

pub fn generate_auth_token(email_address: &str, refresh_token: &str, is_trusted: bool) -> Outcome {
    let auth_token = build_auth_token(is_trusted, email_address).unwrap();

    Outcome::TokensRefreshed(TokenPair::new(refresh_token, &auth_token))
}

pub fn sleep_server(duration: Duration) {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(tokio::time::sleep(duration))
    });
}

pub async fn verify_user_header(user: Option<String>) -> Outcome {
    let method: Method = match extract().await {
        Ok(method) => method,
        Err(_) => {println!("doesnt have method"); return Outcome::VerificationFailure},
    };
    if method == Method::GET {
        match user {
            Some(email) => return Outcome::VerificationSuccess(email.to_string()),
            None => {println!("method was GET but user wasnt passed to function"); return Outcome::VerificationFailure},
        }
    };
    println!("verifying user header");
    let headers: HeaderMap = match extract().await {
        Ok(hello) => hello,
        Err(_) => {println!("doesnt have headers"); return Outcome::VerificationFailure},
    };
    let email = match headers.get(USER_CLAIM_AUTH) {
        Some(header) => header.to_str().unwrap_or_default(),
        None => {println!("doesnt have header"); return Outcome::VerificationFailure},
    };

    if email.is_empty() {
        return Outcome::VerificationFailure;
    } else if !!!email.contains('@') {
        return Outcome::VerificationFailure;
    } else if email.len() < 4 {
        return Outcome::VerificationFailure;
    }

    Outcome::VerificationSuccess(email.to_string())
}
