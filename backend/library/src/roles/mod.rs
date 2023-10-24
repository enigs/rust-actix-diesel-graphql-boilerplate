use async_graphql::Context;
use macros::SetEnum;

#[derive(Default, Debug, Clone, Copy, PartialEq, SetEnum)]
#[derive(diesel::expression::AsExpression, diesel::FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum Role {
    Controller,
    Admin,
    #[default]
    Guest
}

impl Role {
    pub fn get(ctx: &Context<'_>) -> Self {
        match ctx.data_opt::<Self>() {
            Some(data) => *data,
            None =>  Self::Guest
        }
    }
}
