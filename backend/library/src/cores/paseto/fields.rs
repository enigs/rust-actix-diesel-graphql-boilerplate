use async_graphql::Object;
use std::borrow::Cow;

use crate::Paseto;

#[Object]
impl Paseto {
    pub async fn app_name(&self) -> Option<Cow<String>> {
        if !self.app_name.is_empty() {
            return Some(Cow::Borrowed(&self.app_name));
        }

        None
    }

    pub async fn access_token_key_unit(&self) -> Option<Cow<i32>> {
        if !self.access_token_key_unit.is_empty() {
            return match self.access_token_key_unit.parse::<i32>() {
                Ok(value) => Some(Cow::Owned(value)),
                Err(_) => Some(Cow::Owned(0)),
            };
        }

        None
    }

    pub async fn access_token_key_time(&self) -> Option<Cow<String>> {
        if !self.access_token_key_time.is_empty() {
            return Some(Cow::Borrowed(&self.access_token_key_time));
        }

        None
    }

    pub async fn access_token_key_signing(&self) -> Option<Cow<String>> {
        if !self.access_token_key_signing.is_empty() {
            return Some(Cow::Borrowed(&self.access_token_key_signing));
        }

        None
    }

    pub async fn refresh_token_key_unit(&self) -> Option<Cow<i32>> {
        if !self.refresh_token_key_unit.is_empty() {
            return match self.refresh_token_key_unit.parse::<i32>() {
                Ok(value) => Some(Cow::Owned(value)),
                Err(_) => Some(Cow::Owned(0)),
            };
        }

        None
    }

    pub async fn refresh_token_key_time(&self) -> Option<Cow<String>> {
        if !self.refresh_token_key_time.is_empty() {
            return Some(Cow::Borrowed(&self.refresh_token_key_time));
        }

        None
    }

    pub async fn refresh_token_key_signing(&self) -> Option<Cow<String>> {
        if !self.refresh_token_key_signing.is_empty() {
            return Some(Cow::Borrowed(&self.refresh_token_key_signing));
        }

        None
    }
}