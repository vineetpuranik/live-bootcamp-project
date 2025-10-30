use std::os::linux::raw::stat;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode, UserStoreError},
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

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    // Handle request based on user's 2FA configuration
    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    // Call the generate_auth_cookie function defined in the auth module.
    // If the function call fails return AuthAPIError::UnexpectedError.
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(c) => c,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (
        updated_jar,
        Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))),
    )
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    // First, we must generate a new random login attempt ID and 2FA code
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    // Store the ID and code in our 2FA code store.
    // Return `AuthAPIError::UnexpectedError` if the operation fails

    if state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    // TODO: send 2FA code via the email client. Return `AuthAPIError::UnexpectedError` if the operation fails.
    let email_client = state.email_client.read().await;
    match email_client
        .send_email(email, "2FA Code", two_fa_code.as_ref())
        .await
    {
        Ok(()) => {}
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    // Return a TwoFactorAuthResponse. The message should be "2FA required".
    let two_factor_auth_response = TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_string(),
    };

    (
        jar,
        Ok((
            StatusCode::PARTIAL_CONTENT,
            Json(LoginResponse::TwoFactorAuth(two_factor_auth_response)),
        )),
    )
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
