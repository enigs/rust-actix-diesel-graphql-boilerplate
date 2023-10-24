pub mod handler;
pub mod root;

pub use handler::*;
pub use root::*;

use async_graphql::{ EmptySubscription, Schema };
use async_graphql::{ extensions::Logger };
use std::sync::Arc;

use library::{Core, sse::Broadcaster};
use library::TokenParserMiddleware;

pub type ProjectSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

pub fn schema(core: &Arc<Core>, sse: &Arc<Broadcaster>) -> ProjectSchema {
    let query = RootQuery;
    let mutation = RootMutation;
    let subscription = EmptySubscription;

    // Initialize builder schema
    Schema::build(query, mutation, subscription)
        .extension(Logger)
        .extension(TokenParserMiddleware)
        .data(Arc::clone(core))
        .data(Arc::clone(sse))
        .finish()
}