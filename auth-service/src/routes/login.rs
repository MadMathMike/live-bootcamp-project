use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password}};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    
    if let Ok(user) = state.user_store.read().await.get_user(&email).await {
        if user.password == password {
            Ok(StatusCode::OK)
        } else {
            Err(AuthAPIError::IncorrectCredentials)    
        }
    } else {
        Err(AuthAPIError::IncorrectCredentials)
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}