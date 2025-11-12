use crate::helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;
use test_helpers::api_test;

#[api_test]
async fn should_return_422_if_malformed_input() {
    // remove app creation as it is done in proc attribute macro

    // Call helper method to generate a random email
    let random_email = get_random_email();

    // test cases for malformed inputs
    let test_cases = [
        serde_json::json!({
            "password": "password1234",
            "required2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "required2FA": true
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({
            "password": "password1234",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[api_test]
async fn should_return_201_if_valid_input() {
    // remove app creation as it is done in proc attribute macro

    // test cases for valid inputs
    let test_cases = [
        serde_json::json!({
            "email": get_random_email(),
            "password": "password@1234",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": get_random_email(),
            "password": "password@1234",
            "requires2FA": false
        }),
        serde_json::json!({
            "email": get_random_email(),
            "password": "password@1234",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": get_random_email(),
            "password": "password@1234",
            "requires2FA": false
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            201,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[api_test]
async fn should_return_400_if_invalid_input() {
    // The signup route should return a 400 HTTP status code if an invalid input is sent
    // The input is considered invalid if :
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters

    // remove app creation as it is done in proc attribute macro

    // Lets create an array of invalid inputs
    let test_cases = [
        serde_json::json!({
            "email": "emaildoesnotcontainat.com",
            "password": "password@1234",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": get_random_email(),
            "password": "small",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
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

#[api_test]
async fn should_return_409_if_email_already_exists() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
