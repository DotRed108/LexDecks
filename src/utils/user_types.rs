use::core::str::FromStr;
use leptos::{leptos_dom::logging::console_log, prelude::*};
use partial_derive::Partial;
use server_fn::ServerFnError;
use struct_field_names::StructFieldNames;
use strum::{Display, EnumIter, IntoEnumIterator};
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::utils::{
    auth_client::AuthClient, 
    cache_db_interface::{get_cache_status, get_user_info, CacheStatus, get_cache_status_client}, 
    database_types::{Asset, DeckList}, date_and_time::current_time_in_seconds, 
    outcomes::Outcome, proceed, 
    shared_truth::{LOCAL_USER_INFO_KEY, CACHE_OUT_OF_DATE_LIMIT, EMAIL_CLAIM_KEY, EXP_CLAIM_KEY, LOCAL_AUTH_TOKEN_KEY, LOCAL_REFRESH_TOKEN_KEY, USER_CLAIM_AUTH, USER_CLAIM_REFRESH, USER_CLAIM_SIGN_UP}, 
    shared_utilities::{clear_user_cache_and_cookies, get_claim, get_cookie_value, get_item_from_local_storage, get_url_query, is_expired, set_token_cookie, store_item_in_local_storage, verify_then_return_outcome, verify_token}, 
    sign_in_lib::{use_refresh_token, TokenPair}
};

/// Server Imports
#[cfg(feature="ssr")]
use crate::utils::{
    shared_truth::IS_TRUSTED_CLAIM,
    dynamo_utils::{get_user, permission_if_good_standing, setup_client, EMAIL_DB_KEY},
    back_utils::{verify_user_header, get_default_pfp, USERS_TABLE, build_auth_token, build_refresh_token},
};
#[cfg(feature="ssr")]
use aws_sdk_dynamodb::{Client, operation::put_item::PutItemError};
#[cfg(feature="ssr")]
use serde_dynamo::to_item;

#[derive(Partial)]
#[derive(Clone, Debug, Default, PartialEq, StructFieldNames, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub phone: String,
    pub pfp: Asset,
    pub lex_name: String,
    pub lex_rank: Rank,
    pub upload_tokens: f64,
    pub active_decks: DeckList,
    pub owned_decks: DeckList,
    pub colab_decks: DeckList,
    pub user_type: UserType,
    pub settings: Settings,
    pub last_login: u64,
    pub standing: Standing,
    pub sign_up_date: u64,
}

impl ToString for UserInfo {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for UserInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let user_info = match serde_json::from_str(s) {
            Ok(info) => info,
            Err(_) => return Err(()),
        };
        Ok(user_info)
    }
}

impl From<PartialUserInfo> for UserInfo {
    fn from(value: PartialUserInfo) -> Self {
        Self {
            email: value.email.clone().unwrap_or_default(),
            phone: value.phone.clone().unwrap_or_default(),
            pfp: value.pfp.clone().unwrap_or_default(),
            lex_name: value.lex_name.clone().unwrap_or_default(),
            lex_rank: value.lex_rank.unwrap_or_default(),
            upload_tokens: value.upload_tokens.unwrap_or_default(),
            active_decks: value.active_decks.clone().unwrap_or_default(),
            owned_decks: value.owned_decks.clone().unwrap_or_default(),
            colab_decks: value.colab_decks.clone().unwrap_or_default(),
            user_type: value.user_type.unwrap_or_default(),
            settings: value.settings.unwrap_or_default(),
            last_login: value.last_login.unwrap_or_default(),
            standing: value.standing.unwrap_or_default(),
            sign_up_date: value.sign_up_date.unwrap_or_default(),
        }
    }
}

impl UserInfo {
    pub const FULL_USER_CACHE_KEY: &'static str = "FullUserInfo";
    pub const EMAIL_CACHE_KEY: &'static str = UserInfo::FIELD_NAMES.email;
    pub const PHONE_CACHE_KEY: &'static str = UserInfo::FIELD_NAMES.phone;
    pub const ACTIVE_DECKS_CACHE_KEY: &'static str = UserInfo::FIELD_NAMES.active_decks;
    pub const OWNED_DECKS_CACHE_KEY: &'static str = UserInfo::FIELD_NAMES.owned_decks;
    pub const COLAB_DECKS_CACHE_KEY: &'static str = UserInfo::FIELD_NAMES.colab_decks;
    pub const LAST_LOGIN_CACHE_KEY: &'static str = UserInfo::FIELD_NAMES.last_login;
    pub const UPLOAD_TOKENS_CACHE_KEY: &'static str = UserInfo::FIELD_NAMES.upload_tokens;
}


#[derive(Clone, Copy, Debug, Default, PartialEq, Display, EnumIter, Serialize, Deserialize)]
pub enum UserType {
    #[default] Basic,
    Premium,
    Founder,
    First100,
}

impl UserType {
    pub fn is_premium_user(&self) -> bool {
        match self {
            UserType::Basic => false,
            _ => true
        }
    }
}

impl FromStr for UserType {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        for variant in Self::iter() {
            if input == &variant.to_string() {
                return Ok(variant);
            }
        }
        Err(())
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum Standing {
    #[default] WUser,
    Suspended(u64),
}

impl FromStr for Standing {

    type Err = ();

    fn from_str(s: &str) -> Result<Standing, Self::Err> {
        let standing = match serde_json::from_str(s) {
            Ok(standin) => standin,
            Err(_) => return Err(()),
        };

        Ok(standing)
    }
}

impl ToString for Standing {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    color_scheme: u8,
}

impl FromStr for Settings {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let settings = match serde_json::from_str(s) {
            Ok(set) => set,
            Err(_) => return Err(()),
        };
        Ok(settings)
    }
}

impl ToString for Settings {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Display, EnumIter, Serialize, Deserialize)]
pub enum Rank {
    #[default] Rank1,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
}

impl FromStr for Rank {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        for variant in Self::iter() {
            if input == &variant.to_string() {
                return Ok(variant);
            }
        }
        Err(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UserState {
    is_authenticated: bool,
    user: String,
    token: String,
    expiration: String,
    pub sign_in_outcome: Outcome,
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

            return UserState {
                is_authenticated: true,
                user,
                token: auth_token.to_owned(),
                expiration,
                sign_in_outcome: Outcome::UserSignedIn(TokenPair::default()),
            };
        }
        UserState::default()
    }

    pub async fn find_token_or_default(user_state: RwSignal<UserState>) -> Self {
        match user_state.get_untracked().sign_in_outcome {
            Outcome::Waiting => match get_cache_status().await {
                CacheStatus::Complete(as_of_date) => {
                    if as_of_date > (current_time_in_seconds() - CACHE_OUT_OF_DATE_LIMIT) {
                        user_state.set(UserState::default());
                        return UserState::default()
                    }
                },
                _ => proceed(),
            },
            _any_other_outcome => proceed(),
        }
        let mut token_pair = TokenPair::default();
        let mut sign_in_outcome = Outcome::UnresolvedOutcome;
        // Check cookie
        if token_pair.get_auth_token().is_empty() {
            let cookie_auth_token = get_cookie_value(LOCAL_AUTH_TOKEN_KEY).await.unwrap_or_default();
            token_pair.set_auth_token(&cookie_auth_token);
        }
        // Check local storage
        if token_pair.get_auth_token().is_empty() {
            let local_storage_auth_token = get_item_from_local_storage(LOCAL_AUTH_TOKEN_KEY).unwrap_or_default();
            token_pair.set_auth_token(&local_storage_auth_token);
        }
        // Check for refresh token in url
        if token_pair.get_refresh_token().is_empty() {
            let url_refresh_token = get_url_query(USER_CLAIM_REFRESH).await.unwrap_or_default();
            if !!!url_refresh_token.is_empty() {
                match verify_then_return_outcome(&url_refresh_token) {
                    Outcome::VerificationSuccess(token) => token_pair.set_refresh_token(&token),
                    any_other_outcome => sign_in_outcome = any_other_outcome,
                }
            }
        }
        // Check for auth token in url
        if token_pair.get_auth_token().is_empty() {
            let url_auth_token = get_url_query(USER_CLAIM_AUTH).await.unwrap_or_default();
            if !!!url_auth_token.is_empty() {
                match verify_then_return_outcome(&url_auth_token) {
                    Outcome::VerificationSuccess(token) => token_pair.set_auth_token(&token),
                    any_other_outcome => sign_in_outcome = any_other_outcome,
                }
            }
        }
        // Check for sign up token in url
        if token_pair.get_auth_token().is_empty() {
            let url_sign_up_token = get_url_query(USER_CLAIM_SIGN_UP).await.unwrap_or_default();
            if !!!url_sign_up_token.is_empty() {
                match verify_then_return_outcome(&url_sign_up_token) {
                    Outcome::VerificationSuccess(token) => match create_user(token).await.unwrap_or(Outcome::CreateUserFailure("Server failure occured".into())) {
                        Outcome::UserCreationSuccess(tokens) => token_pair = tokens,
                        any_other_outcome => sign_in_outcome = any_other_outcome,
                    },
                    any_other_outcome => sign_in_outcome = any_other_outcome,
                }
            }
        }
        console_log(&format!("{}", sign_in_outcome.to_string()));
        // Check for refresh token in cookie then use it to get an auth token
        if token_pair.get_auth_token().is_empty() || token_pair.get_refresh_token().is_empty() {
            let cookie_refresh_token = get_cookie_value(LOCAL_REFRESH_TOKEN_KEY).await.unwrap_or_default();
            token_pair.set_refresh_token(&cookie_refresh_token);
            if token_pair.get_auth_token().is_empty() {
                let cookie_refresh_outcome = match verify_then_return_outcome(&cookie_refresh_token) {
                    Outcome::VerificationSuccess(token) => use_refresh_token(token).await.unwrap_or_default(),
                    any_other_outcome => any_other_outcome,
                };
                match cookie_refresh_outcome {
                    Outcome::TokensRefreshed(tokens) => token_pair = tokens,
                    _any_other_outcome => {
                        sign_in_outcome = match sign_in_outcome {
                            Outcome::UnresolvedOutcome => Outcome::RefreshTokenFailure("Could not refresh token".into()),
                            any_other_outcome => any_other_outcome,
                        };
                    },
                }
            }
        }
        // Check for refresh token in local storage then use it to get an auth token
        if token_pair.get_auth_token().is_empty() || token_pair.get_refresh_token().is_empty() {
            let local_storage_refresh_token = get_item_from_local_storage(LOCAL_REFRESH_TOKEN_KEY).unwrap_or_default();
            token_pair.set_refresh_token(&local_storage_refresh_token);
            if token_pair.get_auth_token().is_empty() {
                let local_storage_refresh_outcome = match verify_then_return_outcome(&local_storage_refresh_token) {
                    Outcome::VerificationSuccess(token) => use_refresh_token(token).await.unwrap_or_default(),
                    any_other_outcome => any_other_outcome,
                };
                match local_storage_refresh_outcome {
                    Outcome::TokensRefreshed(tokens) => token_pair = tokens,
                    _any_other_outcome => {
                        sign_in_outcome = match sign_in_outcome {
                            Outcome::UnresolvedOutcome => Outcome::RefreshTokenFailure("Could not refresh token".into()),
                            any_other_outcome => any_other_outcome,
                        };
                    },
                }
            }
        }

        // Storing the token_pair
        let auth_successful = if verify_token(&token_pair.get_auth_token()).is_ok() {
            let stored_locally = store_item_in_local_storage(LOCAL_AUTH_TOKEN_KEY, &token_pair.get_auth_token()).is_ok();
            set_token_cookie(&token_pair.get_auth_token()).is_ok() || stored_locally
        } else {
            false
        };
    
        let _refresh_successful = if verify_token(&token_pair.get_refresh_token()).is_ok() {
            let stored_locally = store_item_in_local_storage(LOCAL_REFRESH_TOKEN_KEY, &token_pair.get_refresh_token()).is_ok();
            set_token_cookie(&token_pair.get_refresh_token()).is_ok() || stored_locally
        } else {
            false
        };

        console_log(&format!("{}", sign_in_outcome.to_string()));

        if auth_successful {
            let mut user_resource = UserState::from_token_or_default(&token_pair.get_auth_token());
            user_resource.sign_in_outcome = Outcome::UserSignedIn(token_pair);
            user_state.set(user_resource.clone());
            return user_resource
        } else {
            let mut user_resource = UserState::default();
            user_resource.sign_in_outcome = sign_in_outcome;
            console_log(&format!("{}", user_resource.sign_in_outcome.to_string()));
            user_state.set(user_resource.clone());
            return user_resource
        }
    }

    pub fn replace_outcome(state: Self, outcome: Outcome) -> Self {
        let mut user_state = state;
        user_state.sign_in_outcome = outcome;
        user_state
    }

    pub fn initial_state() -> Self {
        let mut init_state = UserState::default();
        init_state.sign_in_outcome = Outcome::Waiting;
        init_state
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

#[server(client=AuthClient)]
pub async fn user_from_dynamo(email: Option<String>) -> Result<Outcome, ServerFnError> {
    let Outcome::VerificationSuccess(email) = verify_user_header(email).await else {return Ok(Outcome::VerificationFailure)};

    let client = setup_client().await;

    let user = match get_user(&client, &email, None).await {
        Outcome::UserFound(user) => user,
        any_other_outcome => return Ok(any_other_outcome),
    };

    match permission_if_good_standing(&user) {
        Outcome::PermissionGranted(_) => proceed(),
        any_other_outcome => return Ok(any_other_outcome),
    };

    Ok(Outcome::UserFound(user))
}

pub fn setup_user() {
    let user_state = RwSignal::new(UserState::default());
    let user_resource = Resource::new_blocking(|| (), move |_| UserState::find_token_or_default(user_state));
    provide_context(user_state);
    provide_context(user_resource);

    let user_info = Resource::new(
        move || {user_state.get()},
        |user_state| get_user_info(user_state)
    );
    provide_context(user_info);

    Effect::new(move || {
        user_resource.refetch();
    });
    Effect::new(move || {
        match user_info.get() {
            Some(user_info) => {
                if user_info != UserInfo::default() {
                    let _ = store_item_in_local_storage(LOCAL_USER_INFO_KEY, &user_info.to_string()).is_ok();
                } else {
                    proceed()
                }
            },
            None => proceed(),
        }
    });
}

pub fn sign_out(user_state: RwSignal<UserState>, user_resource: Resource<UserState>) {
    let mut sign_out_state = UserState::default();
    sign_out_state.sign_in_outcome = Outcome::UserSignedOut;
    clear_user_cache_and_cookies();
    user_state.set(sign_out_state.clone());
    user_resource.set(Some(sign_out_state));
}

#[server]
async fn create_user(token: String) -> Result<Outcome, ServerFnError> {
    println!("create user");
    let Ok(trusted_token) = verify_token(&token) else {return Ok(Outcome::VerificationFailure)};

    let trusted_device = match get_claim(&trusted_token, IS_TRUSTED_CLAIM) {
        Some(claim) => claim.parse().unwrap_or(false),
        None => false,
    };
    let Some(user_email) = get_claim(&trusted_token, USER_CLAIM_SIGN_UP) else {return Ok(Outcome::VerificationFailure)};
    
    let dynamo_client = setup_client().await;

    let outcome = add_user_to_db(&dynamo_client, &user_email, trusted_device).await;

    Ok(outcome)
}

#[cfg(feature = "ssr")]
async fn add_user_to_db(dynamo_client: &Client, user_email: &str, trusted_device: bool) -> Outcome {
    let Ok(token_pair) = create_token_pair(user_email, trusted_device).await else {
        return Outcome::CreateUserFailure("Could not create token pair".to_string());
    };

    let mut user = UserInfo::default();
    let current_time = current_time_in_seconds();


    user.email = user_email.to_string();
    user.pfp = get_default_pfp();
    user.sign_up_date = current_time;
    user.last_login = current_time;
    user.lex_name = "Lex".to_string();
    
    let item = match to_item(user) {
        Ok(itm) => {itm},
        Err(e) => return Outcome::CreateUserFailure(e.to_string()),
    };
    
    match dynamo_client.put_item().table_name(USERS_TABLE).set_item(Some(item))
    .condition_expression(format!("attribute_not_exists({EMAIL_DB_KEY})")).send().await {
        Ok(_) => proceed(),
        Err(e) => {
            match e.into_service_error() {
                PutItemError::ConditionalCheckFailedException(_) => return Outcome::EmailAlreadyInUse,
                error => return Outcome::CreateUserFailure(error.to_string()),
            }
        },
    }

    Outcome::UserCreationSuccess(token_pair)
}

#[cfg(feature = "ssr")]
async fn create_token_pair(email_address: &str, trusted_device: bool) -> Result<TokenPair, Error> {
    let auth_token = build_auth_token(trusted_device, email_address)?;
    let refresh_token = build_refresh_token(trusted_device, email_address)?;

    Ok(TokenPair::new(&refresh_token, &auth_token))
}
#[derive(Clone, Copy, PartialEq)]
pub enum UpdateUser {
    Clear,
    Fetch,
}
