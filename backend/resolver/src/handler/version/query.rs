use async_graphql::Object;

#[derive(Default)]
pub struct VersionQuery;

#[Object]
impl VersionQuery {
    async fn display(&self) -> String {
        format!("Query Version: {}", env!("CARGO_PKG_VERSION"))
    }
}