use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl User {
    fn new(&self, email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}
