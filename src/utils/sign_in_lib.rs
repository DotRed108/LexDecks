use::core::str::FromStr;

use serde::{Deserialize, Serialize};

use super::shared_truth::SEPARATOR;

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