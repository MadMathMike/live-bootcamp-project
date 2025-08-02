use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
    utils::generate_auth_cookie,
};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials);
    if email.is_err() {
        return (jar, Err(email.err().unwrap()));
    }
    let email = email.ok().unwrap();

    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials);
    if password.is_err() {
        return (jar, Err(password.err().unwrap()));
    }
    let password = password.ok().unwrap();

    let user_result = state.user_store.read().await.get_user(&email).await;

    if user_result.is_err() || user_result.ok().unwrap().password != password {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let auth_cookie = generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError);
    if auth_cookie.is_err() {
        return (jar, Err(auth_cookie.err().unwrap()));
    }
    let auth_cookie = auth_cookie.ok().unwrap();

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
