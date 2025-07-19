use std::error::Error;

use axum::{response::IntoResponse, serve::Serve, Router};
use reqwest::StatusCode;
use tower_http::services::ServeDir;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", axum::routing::post(signup_handler))
            .route("/login", axum::routing::post(login_handler))
            .route("/verify-2fa", axum::routing::post(verify_2fa_handler))
            .route("/logout", axum::routing::post(logout_handler))
            .route("/verify-token", axum::routing::post(verify_token_handler)); 

        let listener = tokio::net::TcpListener::bind(address).await?;
        println!("listening on {}", listener.local_addr().unwrap());

        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application {
            server,
            address: address.to_owned()
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

async fn signup_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn login_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn verify_2fa_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn logout_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn verify_token_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}