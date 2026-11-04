# Rust Bootcamp Project - User Auth Microservice
This repo contains two web services (microservices) written in Rust with the [Axum](https://github.com/tokio-rs/axum) library. Together they demonstrate user authentication:

- **auth-service**: Includes routes for signup, login, logout, email-based two-factor authentication, and JWT verification. Users are stored in Postgres while 2FA codes and banned JWTs are stored in Redis. Emails are sent through Postmark.
- **app-service**: A sample web application with a `/protected` route, that checks the JWT against the auth service before serving its content.

# IMPORTANT
The point of this repository is *not* to demonstrate how to build an authentication microservice. This repository is meant to demonstrate some level of capability in writing (*in Rust*) and deploying microservice-style APIs using standards and practices that are common to the industry. Here are some Rust practices that this repo demonstrates (AI generated list):

- **Type-driven design**
  - *Parse, don't validate*: Email, Password, LoginAttemptId and TwoFACode are newtypes with private fields,
    built through parse(). Once a function receives an Email, it knows the value is valid
    (src/domain/email.rs).
  - *Standard traits*: AsRef<str> gives read access to the inner value, and Default generates fresh IDs and
    codes (src/domain/data_stores.rs).
- **Abstraction and shared state**
  - *Trait-based dependencies*: UserStore, BannedTokenStore, TwoFACodeStore and EmailClient are traits, so
    handlers don't depend on a specific backend. That's why the HashMap and Postgres/Redis stores are
    interchangeable.
  - *Explicit shared state*: AppState holds Arc<RwLock<dyn Trait + Send + Sync>> values and passes them in
    through Axum's State, rather than using globals (src/app_state.rs).
- **Error handling**
  - *Typed errors at boundaries*: thiserror enums define the known failures, and color-eyre's Report carries
    unexpected ones with their context (src/domain/error.rs).
  - *Safe API responses*: IntoResponse for AuthAPIError logs the full error chain, but the client only sees a
    generic "Unexpected error" (src/lib.rs).
- **Security**
  - *Secret handling*: secrecy::Secret keeps passwords and 2FA codes out of Debug output, and reading them
    requires an explicit expose_secret() call.
  - *Password hashing*: Passwords are hashed with Argon2id and a random salt per password.
  - *SQL safety*: sqlx::query! uses bound parameters and checks queries at compile time, with SQLX_OFFLINE set
    for Docker builds.
- **Async and observability**
  - *No blocking the runtime*: Argon2 work is CPU-heavy, so it runs in spawn_blocking. The current tracing span
    is passed in so logs stay connected (src/services/data_stores/postgres_user_store.rs).
  - *Structured tracing*: Each request gets a span with a unique ID, and #[tracing::instrument(skip_all)] keeps
    arguments, including secrets, out of the logs (src/utils/tracing.rs).
- **Testing and project structure**
  - *Layered tests*: Unit tests sit next to the code, and API integration tests live in tests/api.
  - *Property-based testing*: quickcheck and fake generate valid inputs for the domain types.
  - *Test isolation*: Each TestApp creates its own UUID-named database and uses wiremock in place of Postmark
    (tests/api/helpers.rs).
  - *Library plus thin binary*: Logic lives in lib.rs, so tests can start the real app. Application::build also
    reports the actual bound address.
  - *Docker builds*: cargo-chef caches dependencies across multi-stage builds, and the final image doesn't
    include the Rust toolchain.

## Setup & Building
```bash
cargo install cargo-watch
cd app-service
cargo build
cd ..
cd auth-service
cargo build
cd ..
```

## Run servers locally (Manually)
#### App service
```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit http://localhost:8000

#### Auth service
```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

visit http://localhost:3000

## Run servers locally (Docker)
```bash
./docker.sh
```

visit http://localhost:8000 and http://localhost:3000

## Dependencies
```bash
docker run --name ps-db -e POSTGRES_PASSWORD=[YOUR_POSTGRES_PASSWORD] -p 5432:5432 -d postgres:15.2-alpine
docker run --name redis-db -p "6379:6379" -d redis:7.0-alpine
```
