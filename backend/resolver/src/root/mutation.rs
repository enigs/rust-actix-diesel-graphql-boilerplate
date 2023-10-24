use async_graphql::Object;

use library::Guard;

#[derive(Default)]
pub struct RootMutation;

#[Object]
impl RootMutation {
    /// Controller setup mutations. Used to configure server setup.
    #[graphql(visible = "Guard::is_controller", guard = "Guard::controller()")]
    async fn setup(&self) -> crate::SetupMutation {
        crate::SetupMutation
    }

    async fn version(&self) -> crate::VersionMutation {
        crate::VersionMutation
    }
}