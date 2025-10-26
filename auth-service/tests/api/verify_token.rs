use auth_service::{domain::Email, utils::auth::generate_auth_cookie};
use reqwest::Url;

use crate::helpers::TestApp;

fn add_valid_auth_cookie(app: &TestApp) -> String {
    let email = Email::parse("test@test.com".to_owned()).expect("valid email");
    let cookie = generate_auth_cookie(&email).expect("generate auth cookie");
    let url = Url::parse(&app.address).expect("parse app url");
    let cookie_string = cookie.to_string();
    app.cookie_jar.add_cookie_str(&cookie_string, &url);

    cookie.value().to_owned()
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    // Lets create an array of malformed inputs
    let test_cases = [
        serde_json::json!({
            "mytoken": "testToken"
        }),
        serde_json::json!({
            "token_123": "testToken"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_token(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let token = add_valid_auth_cookie(&app);
    println!("{} in should_return_200_valid_token", token);

    // Lets create an array of valid inputs
    let test_cases = [serde_json::json!({
        "token": token
    })];

    for test_case in test_cases.iter() {
        let response = app.post_verify_token(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            200,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // Lets create an array of invalid inputs
    let test_cases = [
        serde_json::json!({
            "token": "invalid1"
        }),
        serde_json::json!({
            "token": "invalid2"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_token(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );
    }
}
