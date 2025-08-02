use crate::helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    // TODO: add a valid user to the test app first?

    let test_cases = [
        // empty JSON
        serde_json::json!({}),
        // missing password
        serde_json::json!({
            "email": "test@example.com"
        }),
        // missing email
        serde_json::json!({
            "password": "password123"
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
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let input = [
        serde_json::json!({
            "email": "",
            "password": "password123"
        }),
        serde_json::json!({
            "email": "invalid_email",
            "password": "password123"
        }),
        serde_json::json!({
            "email": random_email,
            "password": ""
        }),
        serde_json::json!({
            "email": "",
            "password": ""
        }),
        serde_json::json!({
            "email": random_email,
            "password": "invalid"
        }),
    ];

    for i in input.iter() {
        let response = app.post_login(i).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let login_body = serde_json::json!({
        "email": "test@example.com",
        "password": "password123"
    });

    let response = app.post_login(&login_body).await;
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
