use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::{query, PgPool};

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, Password, User,
};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let password_hash = compute_password_hash(user.password.as_ref().to_owned())
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        query!(
            "INSERT INTO users(email, password_hash, requires_2fa) VALUES ($1, $2, $3) ",
            user.email.as_ref(),
            password_hash,
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)
        .map(|_| Ok(()))?
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let user = query!("SELECT * FROM users WHERE email = $1", email.as_ref())
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        match user {
            Some(record) => Ok(User {
                email: Email::parse(record.email).unwrap(),
                // TODO: this can't be right...
                password: Password::parse("garbage1234".to_owned()).unwrap(),
                requires_2fa: record.requires_2fa,
            }),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user_record = query!("SELECT * FROM users WHERE email = $1", email.as_ref())
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        match user_record {
            Some(record) => {
                let password_verification_result =
                    verify_password_hash(record.password_hash, password.as_ref().to_owned()).await;
                if password_verification_result.is_ok() {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tokio::task::spawn_blocking(move || {
        let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&expected_password_hash)?;

        Argon2::default()
            .verify_password(password_candidate.as_bytes(), &expected_password_hash)
            .map_err(|e| e.into())
    })
    .await?
}

async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    tokio::task::spawn_blocking(move || {
        let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

        Ok(password_hash)
    })
    .await?
}
