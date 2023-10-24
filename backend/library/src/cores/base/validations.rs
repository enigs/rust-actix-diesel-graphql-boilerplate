use async_graphql::{Context, Result};

use crate::{BaseForm, BaseError};
use crate::Core;
use crate::Errors;
use crate::Validator;
use crate::Response;

impl BaseForm {
    pub fn validate(&mut self, ctx: &Context<'_>) -> Result<&mut Self> {
        let locale = Core::locales(ctx)?;
        let data = self.sanitize();

        let error = BaseError {
            api_url: Validator::new(locale, "base-api-url")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.api_url)
                .validate_string(),
            web_url: Validator::new(locale, "base-web-url")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.web_url)
                .validate_string(),
            admin_url: Validator::new(locale, "base-admin-url")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.admin_url)
                .validate_string()
        };

        let response = Response::BadRequest;

        match error.is_empty() {
            true => Ok(data),
            false => Err(Errors::to(response, error))
        }
    }
}