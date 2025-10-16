use crate::domain::{Email, Password};

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}
