use async_graphql::{ Context, Guard as AsyncGQLGuard, Result };

use crate::Claims;
use crate::Errors;
use crate::{ExpiredToken, InvalidToken};
use crate::Response;
use crate::Role;

const EXPIRED: &str = "We're sorry, but your authentication token has expired. Please sign in again to continue.";
const FORBIDDEN: &str = "Sorry, you do not have the required permissions to perform this action.";

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Guard {
    authentication: bool,
    role: Option<Vec<Role>>,
}

#[async_graphql::async_trait::async_trait]
impl AsyncGQLGuard for Guard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        // Retrieve role
        let role = match ctx.data_opt::<Role>() {
            Some(role) => *role,
            None => Role::Guest
        };

        // Retrieve invalid & expired states of token
        let is_invalid = ctx.data_opt::<InvalidToken>().is_some();
        let is_expired = ctx.data_opt::<ExpiredToken>().is_some();

        // Contains role
        let roles = match self.role.clone() {
            None => vec![],
            Some(roles) => roles
        };

        // Retrieve claims
        let claims = match ctx.data_opt::<Claims>() {
            Some(claims) => claims.clone(),
            None => Claims::default()
        };

        // Check if not authentication and contains role
        if !self.authentication && roles.contains(&role) {
            return Ok(());
        }

        // Check if authentication and contains claims
        if self.authentication && !claims.is_empty()
            && !is_expired && !is_invalid {
            let role = claims.role
                .unwrap_or_default();

            if roles.contains(&role) {
                return Ok(());
            }
        }

        // Check if authentication and is expired
        if self.authentication && is_expired {
            return Err(Errors::to(Response::Unauthorized, EXPIRED));
        }

        // Return forbidden
        Err(Errors::to(Response::Forbidden, FORBIDDEN))
    }
}

impl Guard {
    pub fn role(role: Vec<Role>) -> Self {
        // Retrieve role
        let role = match role.is_empty() {
            true => None,
            false => Some(role)
        };

        // Return new role guard
        Self {
            role,
            ..Default::default()
        }
    }

    pub fn controller() -> Self {
        Self::role(vec![Role::Controller])
    }

    pub fn is_controller(ctx: &Context) -> bool {
        Role::get(ctx) == Role::Controller
    }

    pub fn admin() -> Self {
        Self::role(vec![Role::Admin])
    }

    pub fn is_admin(ctx: &Context) -> bool {
        Role::get(ctx) == Role::Admin
    }
}

