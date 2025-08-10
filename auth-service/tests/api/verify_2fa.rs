use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode},
    utils::constants::JWT_COOKIE_NAME,
};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({}),
        serde_json::json!({
            "loginAttemptId": "string",
            "2FACode": "string"
        }),
        serde_json::json!({
            "email": random_email,
            "2FACode": "string"
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "string"
        }),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

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
    let random_uuid = uuid::Uuid::new_v4();

    let test_cases = [
        serde_json::json!({
            "email": "invalid_email@",
            "loginAttemptId": random_uuid,
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "invalid_uuid",
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": random_uuid,
            "2FACode": "invalid"
        }),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let login_attempt_id = uuid::Uuid::new_v4().to_string();
    let code = "123456";

    // put 2fa code and login attempt id in the store under the specified email
    app.two_fa_code_store
        .write()
        .await
        .add_code(
            Email::parse(email.clone()).unwrap(),
            LoginAttemptId::parse(login_attempt_id.clone()).unwrap(),
            TwoFACode::parse("123456".to_owned()).unwrap(),
        )
        .await
        .unwrap();

    let test_cases = [
        // email not in 2FA store
        serde_json::json!({
            "email": get_random_email(),
            "loginAttemptId": login_attempt_id,
            "2FACode": code
        }),
        // incorrect login attempt id
        serde_json::json!({
            "email": email,
            "loginAttemptId": uuid::Uuid::new_v4().to_string(),
            "2FACode": code
        }),
        // invalid 2FA code
        serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code.chars().rev().collect::<String>()
        }),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let login_attempt_id = uuid::Uuid::new_v4().to_string();
    let code = "123456";

    app.two_fa_code_store
        .write()
        .await
        .add_code(
            Email::parse(email.clone()).unwrap(),
            LoginAttemptId::parse(login_attempt_id.clone()).unwrap(),
            TwoFACode::parse("123456".to_owned()).unwrap(),
        )
        .await
        .unwrap();

    let request = serde_json::json!({
        "email": email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
    });

    let response = app.post_verify_2fa(&request).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_401_if_old_2fa_code() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let login_attempt_id = uuid::Uuid::new_v4().to_string();
    let code = "123456";

    app.two_fa_code_store
        .write()
        .await
        .add_code(
            Email::parse(email.clone()).unwrap(),
            LoginAttemptId::parse(login_attempt_id.clone()).unwrap(),
            TwoFACode::parse("123456".to_owned()).unwrap(),
        )
        .await
        .unwrap();

    let request = serde_json::json!({
        "email": email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
    });

    let response = app.post_verify_2fa(&request).await;
    assert_eq!(response.status().as_u16(), 200);

    // The token should be removed from the store on the first verification
    // making it inelligible for use a second time
    let response = app.post_verify_2fa(&request).await;
    assert_eq!(response.status().as_u16(), 401);
}
