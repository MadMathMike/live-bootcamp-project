use std::sync::Arc;

use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    Email,
};

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let json_str = serde_json::to_string(&(login_attempt_id, code))
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        self.conn
            .write()
            .await
            .set_ex(get_key(&email), json_str, TEN_MINUTES_IN_SECONDS)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)
            .map(Ok)?
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.conn
            .write()
            .await
            .del(get_key(email))
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)
            .map(Ok)?
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let key = get_key(email);
        let tuple_str = self
            .conn
            .write()
            .await
            .get::<_, String>(key)
            .map_err(|_| TwoFACodeStoreError::LoginAttemptIdNotFound)?;
        Ok(serde_json::from_str::<(LoginAttemptId, TwoFACode)>(&tuple_str)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?)
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
}
