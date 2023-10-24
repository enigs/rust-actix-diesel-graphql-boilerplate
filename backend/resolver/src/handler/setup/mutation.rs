use async_graphql::{Context, Object, Result};
use serde_json::json;
use std::sync::Arc;

use library::Core;
use library::{Errors, Response};
use library::{Base, BaseForm};
use library::{Mailer, MailerForm};
use library::{Paseto, PasetoForm};
use library::{S3, S3Form};

#[derive(Default)]
pub struct SetupMutation;

#[Object]
impl SetupMutation {
    async fn base(&self, ctx: &Context<'_>, mut form: BaseForm) -> Result<Base> {
        // Validate form and convert it to Base struct if it's valid
        let mut form = form.validate(ctx)?
            .to::<Base>();

        // Get database manager
        let manager = Core::database(ctx)?;

        // Upsert base and update current base
        form.upsert(manager)
            .map_err(|e| Errors::to(Response::BadRequest, e.to_string()))?;

        // Parse core settings
        if let Some(settings) = ctx.data_opt::<Arc<Core>>() {
            if let Ok(mut settings) = settings.base.try_write() {
                settings.mutate(&form);

                return Ok(settings.clone());
            }
        }

        // Retrieve locales and set error
        let locale = Core::locales(ctx)?;
        let error = locale.lookup("base-update-failed");
        let response = Response::InternalServerError;

        // Return error
        Err(Errors::to(response, error))
    }

    async fn mailer(&self, ctx: &Context<'_>, mut form: MailerForm, send_to: String) -> Result<Mailer> {
        // Validate form and convert it to Mailer struct if it's valid
        let mut form = form.validate(ctx)?
            .to::<Mailer>();

        // Retrieve locale
        let locale = Core::locales(ctx)?;

        // Retrieve mailer variables
        let action = locale.lookup("mailer-action");
        let app_name = config::APP_NAME;
        let service = form.service.clone().to_uppercase();

        let from = config::MAILER_FROM_NO_REPLY;
        let to = send_to;
        let subject = locale.lookup_with_args(
            "mailer-subject",
            &[("module", form.service.as_str())]
        );

        // Send email
        form.set_template("emails/setup/config.html")
            .set_context(json!({
                "action": action,
                "app_name": app_name,
                "service": service,
                "web_url": Core::base(ctx)?.get_web_url(),
            }))
            .send(from, to, subject)?;

        // Get database manager
        let manager = Core::database(ctx)?;

        // Upsert mailer and update current mailer
        form.upsert(manager)
            .map_err(|e| Errors::to(Response::BadRequest, e.to_string()))?;

        // Parse core settings
        if let Some(settings) = ctx.data_opt::<Arc<Core>>() {
            if let Ok(mut settings) = settings.mailer.try_write() {
                settings.mutate(&form);

                return Ok(settings.clone());
            }
        }

        // Retrieve locales and set error
        let locale = Core::locales(ctx)?;
        let error = locale.lookup("mailer-update-failed");
        let response = Response::InternalServerError;

        // Return error
        Err(Errors::to(response, error))
    }

    async fn paseto(&self, ctx: &Context<'_>, mut form: PasetoForm) -> Result<Paseto> {
        // Validate form and convert it to Paseto struct if it's valid
        let mut form = form.validate(ctx)?
            .to::<Paseto>();

        // Get database manager
        let manager = Core::database(ctx)?;

        // Upsert paseto and update current paseto
        form.upsert(manager)
            .map_err(|e| Errors::to(Response::BadRequest, e.to_string()))?;

        // Parse core settings
        if let Some(settings) = ctx.data_opt::<Arc<Core>>() {
            if let Ok(mut settings) = settings.paseto.try_write() {
                settings.mutate(&form);

                return Ok(settings.clone());
            }
        }

        // Retrieve locales and set error
        let locale = Core::locales(ctx)?;
        let error = locale.lookup("paseto-update-failed");
        let response = Response::InternalServerError;

        // Return error
        Err(Errors::to(response, error))
    }

    async fn s3(&self, ctx: &Context<'_>, mut form: S3Form, send_to: String) -> Result<S3> {
        // Validate form and convert it to S3 struct if it's valid
        let mut form = form.validate(ctx)?
            .to::<S3>();

        // Send test upload
        form.test_image_upload()
            .await?;

        // Get database manager
        let manager = Core::database(ctx)?;

        // Upsert base and update current base
        form.upsert(manager)
            .map_err(|e| Errors::to(Response::BadRequest, e.to_string()))?;

        // Retrieve locale
        let locale = Core::locales(ctx)?;

        // Parse core settings
        if let Some(settings) = ctx.data_opt::<Arc<Core>>() {
            if let Ok(mut settings) = settings.s3.try_write() {
                settings.mutate(&form);

                // Retrieve mailer variables
                let action = locale.lookup("s3-action");
                let app_name = config::APP_NAME;

                let from = config::MAILER_FROM_NO_REPLY;
                let to = send_to;
                let subject = locale.lookup_with_args(
                    "mailer-subject",
                    &[("module", "S3")]
                );

                // Send email
                Core::mailer(ctx)?
                    .set_template("emails/setup/config.html")
                    .set_context(json!({
                        "action": action,
                        "app_name": app_name,
                        "service": "S3",
                        "web_url": Core::base(ctx)?.get_web_url(),
                    }))
                    .send(from, to, subject)?;

                return Ok(settings.clone());
            }
        }

        // Retrieve Set errors
        let error = locale.lookup("s3-update-failed");
        let response = Response::InternalServerError;

        // Return error
        Err(Errors::to(response, error))
    }
}