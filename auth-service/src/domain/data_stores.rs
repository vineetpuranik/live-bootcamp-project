use rand::Rng;
use uuid::Uuid;

use super::{Email, Password, User};

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn store_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn check_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    UnexpectedError,
}

// This trait represents the interface all concrete 2FA code stores should represent
#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        // Use the parse_str function from the uuid crate to ensure id is a valid UUID
        match Uuid::parse_str(&id) {
            Ok(_) => Ok(Self(id)),
            Err(_) => Err("Invalid UUID".into()),
        }
    }
}
impl Default for LoginAttemptId {
    fn default() -> Self {
        // Use the `uuid` crate to generate a random version 4 UUID
        Self(Uuid::new_v4().to_string())
    }
}

// Rust does not know how to turn LoginAttemptId into a &str
// that is why we implement the AsRef<str> trait
// “This type can give you a &str view of itself.”
// “Whenever someone needs a &str, they can call as_ref() on this type to get it.”
impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        // Ensure `code` is a valid 6-digit code
        if code.len() == 6 && code.chars().all(|c| c.is_ascii_digit()) {
            Ok(Self(code))
        } else {
            Err("Not valid 6-digit code".into())
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        // Use the `rand` crate to generate a random 2FAcode.
        // The code should be 6 digits (ex: 834629)
        let rand_6digits = rand::thread_rng().gen_range(100_000..=999_999);
        Self(rand_6digits.to_string())
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
