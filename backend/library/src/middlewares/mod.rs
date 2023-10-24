use async_graphql::{ Request, ServerResult };
use async_graphql::async_trait::async_trait;
use async_graphql::extensions::{ Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest };
use std::sync::{ Arc, RwLock };

use crate::ciphers;
use crate::{BearerToken, ExpiredToken, InvalidToken};
use crate::Claims;
use crate::Paseto;
use crate::Role;

pub struct TokenParserMiddleware;

impl ExtensionFactory for TokenParserMiddleware {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(TokenParserMiddlewareExtension)
    }
}

pub struct TokenParserMiddlewareExtension;

#[async_trait]
impl Extension for TokenParserMiddlewareExtension {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        // Retrieve bearer token
        let token = match ctx.data_opt::<BearerToken>() {
            Some(token) => token.to_string(),
            None => String::new()
        };

        // Set master
        let master = std::env::var("MASTER_KEY")
            .expect("MASTER_KEY is not set");

        // Check role
        let role = match ciphers::decrypt(&token).ok() == Some(master) {
            true => Role::Controller,
            false => Role::Guest
        };

        // Set mutable request
        let mut request = request.data(role);

        // Check if request is Guest
        if matches!(role, Role::Guest) {
            if let Some(paseto) = ctx.data_opt::<Arc<RwLock<Paseto>>>() {
                if let Ok(paseto) = paseto.try_read() {
                    let paseto = paseto.clone();

                    // Validate access token
                    match paseto.validate_access_token::<Claims>(&token) {
                        Ok(claims) => if !claims.is_empty() {
                            request = request.data(claims.role.unwrap_or_default()).data(claims);
                        },
                        Err(error) => match error.to_string().to_lowercase().as_str() {
                            "your authentication token has expired" |
                            "your refresh token has expired" => request = request.data(role).data(ExpiredToken::default()),
                            _ => request = request.data(role).data(InvalidToken::default())
                        }
                    }
                }
            }
        }

        // Continue
        next.run(ctx, request).await
    }
}