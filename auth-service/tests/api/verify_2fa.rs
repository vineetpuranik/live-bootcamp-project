use crate::helpers::{get_random_email, TestApp};
use auth_service::domain::LoginAttemptId;

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
