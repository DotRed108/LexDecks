use::core::str::FromStr;
// use std::{future::Future, pin::Pin, task::{Context, Poll}};
use partial_derive::Partial;
use struct_field_names::StructFieldNames;
use strum::{Display, EnumIter, IntoEnumIterator};
use serde::{Deserialize, Serialize};

use super::database_types::{Asset, DeckList};

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

// impl Future for UserInfo {
//     type Output = UserInfo;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         todo!()
//     }
// }

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