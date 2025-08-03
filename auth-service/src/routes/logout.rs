use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    jar: CookieJar,
    State(state): State<AppState>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = jar.get(JWT_COOKIE_NAME);
    if cookie.is_none() {
        return (jar, Err(AuthAPIError::MissingToken));
    }
    let cookie = cookie.unwrap();

    let token = cookie.value().to_owned();

    let token_validation = validate_token(state.banned_token_store.clone(), &token).await;

    if token_validation.is_err() {
        return (jar, Err(AuthAPIError::InvalidToken));
    }

    let jar = jar.remove(JWT_COOKIE_NAME);

    let mut banned_token_store = state.banned_token_store.write().await;
    // TODO: If we fail to add the token to the banned token store, should logout fail?
    let _ = banned_token_store.add_token(token).await;

    (jar, Ok(StatusCode::OK))
}
