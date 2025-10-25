use crate::helpers::TestApp;
use auth_service::{
    domain::Email,
    utils::{auth::generate_auth_cookie, constants::JWT_COOKIE_NAME},
    ErrorResponse,
};
use reqwest::Url;

fn add_valid_auth_cookie(app: &TestApp) {
    let email = Email::parse("test@test.com".to_owned()).expect("valid email");
    let cookie = generate_auth_cookie(&email).expect("generate auth cookie");
    let url = Url::parse(&app.address).expect("parse app url");
    app.cookie_jar.add_cookie_str(&cookie.to_string(), &url);
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    // add missing cookie
    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!("SameSite=Lax; Secure; Path=/"),
        &Url::parse("http://127.0.0.1").expect("Failed to parse url"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse url"),
    );

    // call logout and assert 401
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    add_valid_auth_cookie(&app);

    // call logout and assert 401
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    add_valid_auth_cookie(&app);

    // call logout twice and assert 400
    let _ = app.post_logout().await;
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}
