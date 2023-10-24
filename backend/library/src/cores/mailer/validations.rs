use async_graphql::{Context, Result};

use crate::{MailerForm, MailerError};
use crate::Core;
use crate::Errors;
use crate::Validator;
use crate::Response;

impl MailerForm {
    pub fn validate(&mut self, ctx: &Context<'_>) -> Result<&mut Self> {
        let locale = Core::locales(ctx)?;
        let data = self.sanitize();

        let error = MailerError {
            username: Validator::new(locale, "mailer-username")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.username)
                .validate_string(),
            password: Validator::new(locale, "mailer-password")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.password)
                .validate_string(),
            smtp_host: Validator::new(locale, "mailer-smtp-host")
                .set_min(3)
                .set_max(100)
                .set_as_required(true)
                .set_string_value(&data.smtp_host)
                .validate_string(),
            service: Validator::new(locale, "mailer-service")
                .set_option_list_string(&["SES", "MAILGUN"])
                .set_as_case_sensitive(false)
                .set_as_required(true)
                .set_string_value(&data.service)
                .validate_list_string()
        };

        let response = Response::BadRequest;

        match error.is_empty() {
            true => Ok(data),
            false => Err(Errors::to(response, error))
        }
    }
}