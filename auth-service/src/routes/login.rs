use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, UserStoreError},
    utils::auth::generate_auth_cookie,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email.clone()) {
        Ok(e) => e,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };
    let password = match Password::parse(request.password.clone()) {
        Ok(p) => p,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    println!("Email is {:?}", email);
    println!("Password is {:?}", password);

    // we need read access to state
    // get exclusive write access to user store and add new_user to user store
    let user_store = state.user_store.read().await;

    match user_store.validate_user(&email, &password).await {
        Ok(()) => {}
        Err(UserStoreError::UserNotFound) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
        Err(UserStoreError::InvalidCredentials) => {
            return (jar, Err(AuthAPIError::IncorrectCredentials))
        }
        _ => return (jar, Err(AuthAPIError::UnexpectedError)),
    }

    // Call the generate_auth_cookie function defined in the auth module.
    // If the function call fails return AuthAPIError::UnexpectedError.
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(c) => c,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
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
