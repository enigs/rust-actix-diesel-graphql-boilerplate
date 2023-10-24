use async_graphql::Object;
use std::borrow::Cow;

use crate::Mailer;

#[Object]
impl Mailer {
    pub async fn username(&self) -> Option<Cow<String>> {
        if !self.username.is_empty() {
            return Some(Cow::Borrowed(&self.username));
        }

        None
    }

    pub async fn password(&self) -> Option<Cow<String>> {
        if !self.password.is_empty() {
            return Some(Cow::Borrowed(&self.password));
        }

        None
    }

    pub async fn smtp_host(&self) -> Option<Cow<String>> {
        if !self.smtp_host.is_empty() {
            return Some(Cow::Borrowed(&self.smtp_host));
        }

        None
    }

    pub async fn service(&self) -> Option<Cow<String>> {
        if !self.service.is_empty() {
            return Some(Cow::Borrowed(&self.service));
        }

        None
    }
}