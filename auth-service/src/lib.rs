pub mod routes;

use std::error::Error;

use axum::{serve::Serve, Router};
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
            .route("/signup", axum::routing::post(routes::signup_handler))
            .route("/login", axum::routing::post(routes::login_handler))
            .route("/verify-2fa", axum::routing::post(routes::verify_2fa_handler))
            .route("/logout", axum::routing::post(routes::logout_handler))
            .route("/verify-token", axum::routing::post(routes::verify_token_handler)); 

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