use std::{str::FromStr, time::{Duration, SystemTime, UNIX_EPOCH}};

use crate::utils_and_structs::database_types::{Asset, S3Address};
use crate::utils_and_structs::user_types::Standing;
use pasetors::{claims::{Claims, ClaimsValidationRules}, errors::Error as PasetoError, keys::{AsymmetricPublicKey, AsymmetricSecretKey}, public, token::{TrustedToken, UntrustedToken}, version4::V4, Public};

use super::{date_and_time::current_time_in_seconds, outcomes::Outcome, shared_truth::{IS_TRUSTED_CLAIM, PUBLIC_KEY, USER_CLAIM_AUTH, USER_CLAIM_REFRESH, USER_CLAIM_SIGN_UP}, sign_in_lib::TokenPair};

pub const PUBLIC_DECKS_TABLE: &str = "LEXDecks";

pub const PUBLIC_DECKS_BUCKET: &str = "lexpublicdecksbucket";

pub const PUBLIC_DECKS_STAGING_BUCKET: &str = "lexpublicdecksstagingbucket";

pub const PFP_BUCKET: &str = "lexpfpbucket";

pub const USERS_TABLE: &str = "LEXUsers";

pub const UPLOAD_TOKEN_PRICE_IN_DOLLARS: f64 = 0.20;

pub const PRIVATE_KEY: [u8; 64] = [120, 216, 201, 41, 128, 44, 57, 121, 70, 69, 82, 58, 153, 198, 197, 246, 43, 23, 205, 194, 157, 95, 74, 144, 87, 150, 238, 23, 49, 149, 175, 118, 183, 177, 157, 57, 78, 176, 181, 67, 152, 166, 91, 120, 67, 99, 14, 16, 189, 46, 30, 75, 88, 77, 182, 203, 206, 212, 82, 16, 179, 151, 71, 24];

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

    let private_key = AsymmetricSecretKey::<V4>::from(&PRIVATE_KEY)?;
    let refresh_token = public::sign(&private_key, &claims, None, Some(b"implicit assertion"))?;

    Ok(refresh_token)
}

pub fn build_sign_up_token(is_trusted: bool, email_address: &str) -> Result<String, PasetoError> {
    let one_hour = 3600;

    let mut claims = Claims::new_expires_in(&Duration::from_secs(one_hour))?;
    claims.add_additional(USER_CLAIM_SIGN_UP, email_address)?;
    claims.add_additional(IS_TRUSTED_CLAIM, is_trusted.to_string())?;

    let private_key = AsymmetricSecretKey::<V4>::from(&PRIVATE_KEY)?;
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

    let private_key = AsymmetricSecretKey::<V4>::from(&PRIVATE_KEY)?;
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
