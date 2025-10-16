use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}