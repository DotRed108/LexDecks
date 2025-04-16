use std::str::FromStr;

use serde::{Deserialize, Serialize};
use super::sign_in_lib::TokenPair;
use super::user_types::{PartialUserInfo, UserInfo};
use super::database_types::UpdateRecipes;

pub const OUTCOME_SEPARATOR: &str = "|x|X|x|X|x|";

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Outcome {
    #[default] UnresolvedOutcome,
    VerificationSuccess(String),
    VerificationFailure,
    TokenExpired,
    UserDoesNotHavePermission,
    InvalidRequest,
    UserSuspended(u64),
    PermissionGranted(String),
    PermissionGrantedReturnUser(UserInfo),

    EmailSendSuccess,
    EmailSendFailure(String),
    EmailAlreadyInUse,
    CreateUserFailure(String),
    UpdateUserFailure(String),
    UserCreationSuccess(TokenPair),
    DatabaseUpdateSuccess(UpdateRecipes),
    SignInTokenPairVerified(TokenPair),
    UserSignedIn,
    UserOnlyHasRefreshToken,
    UserNotSignedIn,
    TokensRefreshed(String),
    RefreshTokenFailure(String),
    NoRefreshTokenFound,
    UserNotFound,
    UserFound(UserInfo),
    PartialUserFound(PartialUserInfo),

    CacheFailed(String),
    CacheSucceeded,
    AlreadyCached(String),

    ItemsNotFound,
    UnspecifiedQueryFailure(String),
    ItemsFound(String),
    AssetRetrieved(Vec<u8>),

    PresignedUrlNotRetrieved(String),
    PresignedUrlRetrieved(String),
    DeckNotUploadedToBucket,
    DeckUploadedToBucket(String),
    DeckProcessed(String),
    DeckCouldNotBeProcessed(String),
    NotEnoughUploadTokens(f64),
    IncorrectType,
    TooManyFiles,
    TooManyDecks,

    NoteUpdateFailed(String),
    NoteUpdateSuccess,

    MultiOutcome(Vec<Outcome>),
}

impl ToString for Outcome {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outcome = match serde_json::from_str(s) {
            Ok(o) => o,
            Err(_) => return Err(()),
        };
        Ok(outcome)
    }
}

impl Outcome {
    pub fn new_multi_outcome(outcome1: Outcome, outcome2: Outcome) -> Outcome {
        Outcome::MultiOutcome(vec![outcome1, outcome2])
    }  

    pub fn add_outcome(&mut self, outcome: Outcome) {
        match self {
            Outcome::MultiOutcome(outcomes) => outcomes.push(outcome),
            _ => {
                let self_copy = self.clone();
                take_mut::take(self, |_| Self::new_multi_outcome(self_copy, outcome))
            },
        };
    }

    pub fn multi_outcome_to_vec(&self) -> Vec<Outcome> {
        let outcomes = match self {
            Outcome::MultiOutcome(outcomes) => outcomes.to_owned(),
            any_other_outcome => {eprintln!("cannot deconstruct as multi outcome"); return vec![any_other_outcome.to_owned()]}
        };

        outcomes
    }
}
