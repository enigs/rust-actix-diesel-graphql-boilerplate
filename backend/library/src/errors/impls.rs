use async_graphql::{ErrorExtensions, Value};
use serde_json::json;

use crate::{Errors, Response};
use crate::errors::result::ErrorResult;

impl Errors {
    pub fn to<T>(response: Response, error: T) -> async_graphql::Error
        where T: serde::Serialize
    {
        let errors = Errors {
            errors: Some(Value::from_json(json!(error)).unwrap_or_default()),
            ..Default::default()
        };

        match response {
            Response::BadRequest => errors.bad_request(),
            Response::Unauthorized => errors.unauthorized(),
            Response::PaymentRequired => errors.payment_required(),
            Response::Forbidden => errors.forbidden(),
            Response::NotFound => errors.not_found(),
            Response::InternalServerError => errors.internal_server_error(),
            Response::ErrorWithoutExtensions => errors.error_without_extensions(),
        }
    }

    fn bad_request(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::BadRequest;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    fn unauthorized(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::Unauthorized;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    fn payment_required(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::PaymentRequired;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    fn forbidden(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::Forbidden;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    fn not_found(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::NotFound;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    fn internal_server_error(&self) -> async_graphql::Error {
        // Set initial error
        let error = ErrorResult::InternalServerError;

        // Check if message is set
        if let Some(message) = &self.message {
            return error.extend_with(|_, e| e.set("error", message.to_string()))
        }

        // Check if errors is set
        if let Some(errors) = &self.errors {
            return error.extend_with(|_, e| e.set("errors", errors.clone()))
        }

        // Return error
        error.extend()
    }

    fn error_without_extensions(&self) -> async_graphql::Error {
        // Set error
        ErrorResult::ErrorWithoutExtensions
            .extend()
    }
}