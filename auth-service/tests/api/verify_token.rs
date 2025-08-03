use auth_service::{domain::{BannedTokenStore, Email}, utils::generate_auth_cookie, ErrorResponse};

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        // empty JSON
        serde_json::json!({}),
        // wrong data type for token field
        serde_json::json!({
            "token": true
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
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "token": "invalid"
    });
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Unauthorized".to_owned()
    );
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;

    let email = Email::parse(String::from("test@example.com")).unwrap();
    let cookie = generate_auth_cookie(&email).unwrap();
    let token = cookie.value();

    app.banned_token_store.write().await.add_token(token.to_owned()).await.unwrap();

    let body = serde_json::json!({
        "token": token
    });
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let email = Email::parse(String::from("test@example.com")).unwrap();
    let cookie = generate_auth_cookie(&email).unwrap();
    let token = cookie.value();
    let body = serde_json::json!({
        "token": token
    });
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status().as_u16(), 200);
}
