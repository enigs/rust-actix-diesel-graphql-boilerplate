use async_graphql::{ Context, Object, Result };

use library::Core;
use library::Base;
use library::Mailer;
use library::Paseto;
use library::S3;

#[derive(Default)]
pub struct SetupQuery;

#[Object]
impl SetupQuery {
    async fn base(&self, ctx: &Context<'_>) -> Result<Base> {
        Ok(Core::base(ctx)?.clone())
    }

    async fn mailer(&self, ctx: &Context<'_>) -> Result<Mailer> {
        Ok(Core::mailer(ctx)?.clone())
    }

    async fn paseto(&self, ctx: &Context<'_>) -> Result<Paseto> {
        Ok(Core::paseto(ctx)?.clone())
    }

    async fn s3(&self, ctx: &Context<'_>) -> Result<S3> {
        Ok(Core::s3(ctx)?.clone())
    }
}