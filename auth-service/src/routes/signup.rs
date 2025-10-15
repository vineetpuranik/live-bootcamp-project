use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User},
    services::UserStoreError,
};

// Use axum's state extractor to pass in AppState
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    // Return AuthAPIError:InvalidCredentials if :
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters
    if email.is_empty() || !email.contains("@") || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // create a new User instance using data int the request
    let new_user = User {
        email: email,
        password: password,
        requires_2fa: request.requires_2fa,
    };

    // get exclusive write access to user store and add new_user to user store
    let mut user_store = state.user_store.write().await;

    // early return AuthAPIError::UserAlreadyExists if add_user returns UserStoreError::UserAlreadyExists
    // early return AuthAPIError::UnexpectedError if add_user fails
    let _ = match user_store.add_user(new_user) {
        Ok(()) => {}
        Err(UserStoreError::UserAlreadyExists) => return Err(AuthAPIError::UserAlreadyExists),
        _ => return Err(AuthAPIError::UnexpectedError),
    };

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}
