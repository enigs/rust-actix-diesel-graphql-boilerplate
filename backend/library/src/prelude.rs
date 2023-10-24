use serde_json::Value;

pub trait CustomClaims: Default {
    fn convert_from(value: Value) -> Self {
        let mut claims = Self::default();

        if let Some(data) = value["aid"].as_str() {
            claims.set_aid(data);
        }

        if let Some(data) = value["sid"].as_str() {
            claims.set_sid(data);
        }

        if let Some(data) = value["role"].as_str() {
            claims.set_role(data);
        }

        if let Some(data) = value["status"].as_str() {
            claims.set_status(data);
        }

        claims
    }

    fn set_aid(&mut self, id: &str) -> &mut Self;

    fn set_sid(&mut self, id: &str) -> &mut Self;

    fn set_role(&mut self, role: &str) -> &mut Self;

    fn set_status(&mut self, status: &str) -> &mut Self;
}