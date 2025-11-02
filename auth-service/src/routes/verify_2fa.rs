use crate::domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

pub async fn verify_2fa(
    Json(request): Json<Verify2FARequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // parse email
    let email = match Email::parse(request.email) {
        Ok(e) => e,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id) {
        Ok(l) => l,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
        Ok(t) => t,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}
