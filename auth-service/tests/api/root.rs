use test_helpers::api_test;

use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[api_test]
async fn root_returns_auth_ui() {
    // remove app creation as it is done in proc attribute macro
    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
