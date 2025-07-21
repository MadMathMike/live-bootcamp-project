use crate::helpers::{TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    //let random_email = get_random_email(); // Call helper method to generate email

    let test_cases = [
        serde_json::json!("garbage"), // String literals are not valid object literal
        serde_json::json!([]), // Array literals are not valie object literals
        serde_json::json!({}), // Empty object
        serde_json::json!({ // Missing email
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({ // Missing requires2FA
            "email": "garbage",
            "password": "password123"
        }),
        serde_json::json!({ // Missing password
            "email": "garbage",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        let status_code =  response.status().as_u16();
        assert_eq!(status_code, 422, "Failed for input: {:?}", test_case);
    }
}