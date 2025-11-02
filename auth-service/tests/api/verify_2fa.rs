use crate::helpers::{get_random_email, TestApp};
use uuid::Uuid;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "email@test.com",
            "loginAttemptId": "test",
        }),
        serde_json::json!({
            "email1": "user@example.com",
            "loginAttemptId": "string",
            "2FACod2e": "string",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
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
    let app = TestApp::new().await;

    let valid_email = get_random_email();
    let valid_login_attempt_id = Uuid::new_v4().to_string();
    let valid_2_facode = "123456".to_string();

    let invalid_email = "test.com".to_string();
    let invalid_login_attempt_id = "InvalidLoginAttemptId".to_string();
    let invalid_2_facode = "Invalid2FA".to_string();

    // Add test cases for invalid inputs so that we can assert 400 response
    // email : add an email without '@'
    // loginAttemptId : add a login attempt id that is not a uuid
    // 2FACode: add a 2fa code that is not a numeric 6 digit code
    let test_cases = [
        serde_json::json!({
            "email": invalid_email,
            "loginAttemptId": valid_login_attempt_id,
            "2FACode": valid_2_facode,
        }),
        serde_json::json!({
            "email": valid_email,
            "loginAttemptId": invalid_login_attempt_id,
            "2FACode": valid_2_facode,
        }),
        serde_json::json!({
            "email": valid_email,
            "loginAttemptId": valid_login_attempt_id,
            "2FACode": invalid_2_facode,
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}
