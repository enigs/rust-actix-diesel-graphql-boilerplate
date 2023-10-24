use async_graphql::Object;

#[derive(Default)]
pub struct VersionMutation;

#[Object]
impl VersionMutation {
    async fn display(&self) -> String {
        format!("Mutation Version: {}", env!("CARGO_PKG_VERSION"))
    }
}
