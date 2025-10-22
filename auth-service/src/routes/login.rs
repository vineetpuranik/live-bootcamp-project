use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, UserStoreError},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    println!("Email is {:?}", email);
    println!("Password is {:?}", password);

    // we need read access to state
    // get exclusive write access to user store and add new_user to user store
    let user_store = state.user_store.read().await;

    match user_store.validate_user(&email, &password).await {
        Ok(()) => {}
        Err(UserStoreError::InvalidCredentials) => return Err(AuthAPIError::InvalidCredentials),
        _ => return Err(AuthAPIError::UnexpectedError),
    }

    let response = Json(LoginResponse {
        message: "Login Successful!".to_string(),
    });
    Ok((StatusCode::OK, response))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}
