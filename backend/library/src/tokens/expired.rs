#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExpiredToken(pub String);

impl ToString for ExpiredToken {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl ExpiredToken {
    pub fn new<T>(token: T) -> Self
        where T: ToString
    {
        Self(token.to_string())
    }
}