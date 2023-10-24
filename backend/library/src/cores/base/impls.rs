use std::borrow::Cow;

use crate::Base;

impl Base {
    pub fn get_api_url(&self) -> Cow<str> {
        Cow::from(&self.api_url)
    }

    pub fn get_web_url(&self) -> Cow<str> {
        Cow::from(&self.web_url)
    }

    pub fn get_admin_url(&self) -> Cow<str> {
        Cow::from(&self.admin_url)
    }
}