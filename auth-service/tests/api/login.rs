use crate::helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;

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
