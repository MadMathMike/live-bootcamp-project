use auth_service::{
    domain::Email,
    utils::{constants::JWT_COOKIE_NAME, generate_auth_cookie},
    ErrorResponse,
};
use reqwest::Url;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Missing auth token".to_owned()
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!("{JWT_COOKIE_NAME}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/"),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
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
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let email = Email::parse(String::from("test@example.com")).unwrap();
    let cookie = generate_auth_cookie(&email).unwrap();
    let cookie = &format!("{JWT_COOKIE_NAME}={}", cookie.value());
    let url = &Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    app.cookie_jar.add_cookie_str(cookie, url);

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    let email = Email::parse(String::from("test@example.com")).unwrap();
    let cookie = generate_auth_cookie(&email).unwrap();
    let cookie = &format!("{JWT_COOKIE_NAME}={}", cookie.value());
    let url = &Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    app.cookie_jar.add_cookie_str(cookie, url);

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}
