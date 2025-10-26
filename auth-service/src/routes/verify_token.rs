use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{domain::AuthAPIError, utils::validate_token};

pub async fn verify_token(
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // validate token
    match validate_token(&request.token).await {
        Ok(_) => {}
        Err(_) => return Err(AuthAPIError::InvalidToken),
    }

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
