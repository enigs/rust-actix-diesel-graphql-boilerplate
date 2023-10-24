use async_graphql::Object;

use library::Guard;

#[derive(Default)]
pub struct RootQuery;

#[Object]
impl RootQuery {
    /// Controller setup queries. View server setup.
    #[graphql(visible = "Guard::is_controller", guard = "Guard::controller()")]
    async fn setup(&self) -> crate::SetupQuery {
        crate::SetupQuery
    }

    async fn version(&self) -> crate::VersionQuery {
        crate::VersionQuery
    }
}