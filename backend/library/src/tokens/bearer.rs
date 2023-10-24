use async_graphql::Context;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BearerToken(pub String);

impl ToString for BearerToken {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl BearerToken {
    pub fn new<T>(token: T) -> Self
        where T: ToString
    {
        Self(token.to_string())
    }

    pub fn get(ctx: &Context<'_>) -> String {
        match ctx.data_opt::<Self>() {
            Some(token) => token.to_string(),
            None => String::new()
        }
    }
}