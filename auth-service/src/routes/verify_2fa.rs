use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

pub async fn verify_2fa(Json(request): Json<Verify2FARequest>) -> impl IntoResponse {
    StatusCode::OK.into_response()
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}