use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{AuthAPIError, Email, Password, User}};

pub async fn signup(
    state: State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).ok_or( AuthAPIError::InvalidCredentials)?;
    let password = Password::parse(request.password).ok_or(AuthAPIError::InvalidCredentials)?;
    
    let mut user_store = state.user_store.write().await;
    
    let existing_user = user_store.get_user(&email).await;
    if existing_user.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists)
    }

    let user = User::new(
        email,
        password,
        request.requires_2fa,
    );

    (user_store.add_user(user).await).map_err(|_| AuthAPIError::UnexpectedError)?;

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}