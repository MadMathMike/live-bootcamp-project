use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::{AuthAPIError, User}};

pub async fn signup(
    state: State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    if request.email.is_empty() 
        || !request.email.contains("@") 
        || request.password.chars().count() < 8 
    {
        return Err(AuthAPIError::InvalidCredentials)
    }

    let user = User::new(
        request.email,
        request.password,
        request.requires_2fa,
    );

    let mut user_store = state.user_store.write().await;
    
    let existing_user = user_store.get_user(&user.email);
    if existing_user.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists)
    }

    user_store.add_user(user).map_err(|_| AuthAPIError::UnexpectedError)?;

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