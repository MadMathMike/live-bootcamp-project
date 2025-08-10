use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode}, utils::auth::generate_auth_cookie,
};

pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let two_fa_code =
        TwoFACode::parse(request.two_fa_code).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let mut two_fa_code_store = state.two_fa_code_store.write().await;

    let code_tuple = two_fa_code_store
        .get_code(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if code_tuple != (login_attempt_id, two_fa_code) {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    two_fa_code_store
        .remove_code(&email)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return Err(AuthAPIError::UnexpectedError),
    };

    let jar = jar.add(auth_cookie);

    Ok((jar, StatusCode::OK.into_response()))
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}
