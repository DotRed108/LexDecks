use::core::str::FromStr;
use leptos::prelude::{server, ServerFnError};

use serde::{Deserialize, Serialize};

use crate::utils::{
    shared_truth::{SEPARATOR, USER_CLAIM_REFRESH, IS_TRUSTED_CLAIM},
    outcomes::Outcome,
    shared_utilities::{verify_token, get_claim}
};

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TokenPair(String, String);

impl TokenPair {
    pub fn get_refresh_token(&self) -> String {
        return self.0.clone()
    }
    pub fn get_auth_token(&self) -> String {
        return self.1.clone()
    }
    pub fn set_refresh_token(&mut self, new_token: &str) {
        self.0 = new_token.to_owned();
    }
    pub fn set_auth_token(&mut self, new_token: &str) {
        self.1 = new_token.to_owned();
    }
    pub fn new(refresh_token: &str, auth_token: &str) -> TokenPair {
        TokenPair(refresh_token.to_owned(), auth_token.to_owned())
    }
}

impl FromStr for TokenPair {

    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(SEPARATOR);

        let Some(refresh_token) = tokens.next() else {
            return Err(());
        };
        let Some(auth_token) = tokens.next() else {
            return Err(());
        };
        
        Ok(TokenPair::new(refresh_token, auth_token))
    }

}

impl ToString for TokenPair {
    fn to_string(&self) -> String {
        format!("{}{}{}", self.get_refresh_token(), SEPARATOR, self.get_auth_token())
    }
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
