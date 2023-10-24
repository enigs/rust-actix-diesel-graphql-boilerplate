use async_graphql::Object;
use std::borrow::Cow;

use crate::Base;

#[Object]
impl Base {
    pub async fn api_url(&self) -> Option<Cow<String>> {
        if !self.api_url.is_empty() {
            return Some(Cow::Borrowed(&self.api_url));
        }

        None
    }

    pub async fn web_url(&self) -> Option<Cow<String>> {
        if !self.web_url.is_empty() {
            return Some(Cow::Borrowed(&self.admin_url));
        }

        None
    }

    pub async fn admin_url(&self) -> Option<Cow<String>> {
        if !self.admin_url.is_empty() {
            return Some(Cow::Borrowed(&self.admin_url));
        }

        None
    }
}