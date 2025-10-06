use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

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
