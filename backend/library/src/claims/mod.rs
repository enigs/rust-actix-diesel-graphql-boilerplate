use serde::{ Serialize, Deserialize };

use crate::prelude::*;
use crate::Errors;
use crate::Role;
use crate::Response;
use crate::Status;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub aid: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl CustomClaims for Claims {
    fn set_aid(&mut self, id: &str) -> &mut Self {
        self.aid = id.to_string();
        self
    }

    fn set_sid(&mut self, id: &str) -> &mut Self {
        self.sid = id.to_string();
        self
    }

    fn set_role(&mut self, role: &str) -> &mut Self {
        self.role = Some(Role::from(role));
        self
    }

    fn set_status(&mut self, status: &str) -> &mut Self {
        self.status = Some(Status::from(status));
        self
    }
}

impl Claims {
    pub fn get(ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Self> {
        let response = Response::InternalServerError;
        let error = "Unable to parse claims from context";

        match ctx.data_opt::<Self>() {
            Some(claims) => Ok(claims.clone()),
            None =>  Err(Errors::to(response, error))
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}