use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, CookieJar};

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the CookieJar
    // Return AuthAPIError::MissingToken if the cookie is not found
    let token = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie.value().to_owned(),
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    // Validate JWT token by calling `validate_token` from the auth service.
    // If the token is valid you can ignore the returned claims for now.
    // Return AuthAPIError::InvalidToken if validation fails.
    match validate_token(&token).await {
        Ok(_) => {}
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    }

    // Remove JWT cookie from the cookie jar
    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));

    (jar, Ok(StatusCode::OK))
}
