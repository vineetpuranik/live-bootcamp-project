use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME, ErrorResponse,
};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    // Call helper method to generate a random email
    let random_email = get_random_email();

    // test cases for malformed inputs
    let test_cases = [
        serde_json::json!({
            "email12": &random_email,
            "password": "true"
        }),
        serde_json::json!({
            "email": &random_email,
            "passwor2": "true"
        }),
        serde_json::json!({
            "email": &random_email,
        }),
        serde_json::json!({
            "password": "password1234",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    // The signup route should return a 400 HTTP status code if an invalid input is sent
    // The input is considered invalid if :
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters

    let app = TestApp::new().await;

    // Lets create an array of invalid inputs
    let test_cases = [
        serde_json::json!({
            "email": "emaildoesnotcontainat.com",
            "password": "password@1234"
        }),
        serde_json::json!({
            "email": "emailcontainat@test.com",
            "password": "small",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid Credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.

    let app = TestApp::new().await;

    // Lets create an array of invalid inputs
    let test_cases = [
        serde_json::json!({
            "email": "email@test.com",
            "password": "password@1234"
        }),
        serde_json::json!({
            "email": "emailcontainat@test.com",
            "password": "regularpassword@1234",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "IncorrectCredentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    // Create a valid sign up request with 2fa enabled
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "longpasswordforme",
        "requires2FA": true,
    });

    // call post sign-up and make sure we get 201 indicating sign up request was processed successfully
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // call login with the new user and make sure 206 is returned since 2FA is enabled for the user
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "longpasswordforme",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    let email = Email::parse(random_email.clone()).expect("Failed to parse email");
    let login_attempt_id = json_body.login_attempt_id.clone();
    let store = app.two_fa_code_store.read().await;
    let (stored_login_attempt_id, _) = store
        .get_code(&email)
        .await
        .expect("Expected login attempt id to be stored for 2FA");

    assert_eq!(stored_login_attempt_id.as_ref(), login_attempt_id.as_str());
}
