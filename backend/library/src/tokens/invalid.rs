#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvalidToken(pub String);

impl ToString for InvalidToken {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl InvalidToken {
    pub fn new<T>(token: T) -> Self
        where T: ToString
    {
        Self(token.to_string())
    }
}