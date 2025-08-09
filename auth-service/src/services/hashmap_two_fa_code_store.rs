use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>{
        self.codes.remove(email);
        Ok(())
    }
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>{
        match self.codes.get(email) {
            Some(code_tuple) => Ok(code_tuple.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse(String::from("test@example.com")).unwrap();
        let uuid = uuid::Uuid::new_v4().to_string();
        let login_attempt_id = LoginAttemptId::parse(uuid).unwrap();
        let code = TwoFACode::parse(String::from("123456")).unwrap();

        let result = store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await;

        assert!(result.is_ok());
        let stored_code = store.codes.remove(&email);
        assert!(stored_code.is_some());
        assert_eq!((login_attempt_id, code), stored_code.unwrap());
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse(String::from("test@example.com")).unwrap();
        let uuid = uuid::Uuid::new_v4().to_string();
        let login_attempt_id = LoginAttemptId::parse(uuid).unwrap();
        let code = TwoFACode::parse(String::from("123456")).unwrap();

        store.codes.insert(email.clone(), (login_attempt_id, code));
        assert!(store.codes.contains_key(&email));

        let result = store.remove_code(&email).await;
        assert!(result.is_ok());

        assert!(!store.codes.contains_key(&email));
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse(String::from("test@example.com")).unwrap();
        let result = store.get_code(&email).await;
        assert!(result.is_err());
        assert_eq!(TwoFACodeStoreError::LoginAttemptIdNotFound, result.err().unwrap());

        let uuid = uuid::Uuid::new_v4().to_string();
        let login_attempt_id = LoginAttemptId::parse(uuid).unwrap();
        let code = TwoFACode::parse(String::from("123456")).unwrap();

        store.codes.insert(email.clone(), (login_attempt_id.clone(), code.clone()));
        let result = store.get_code(&email).await;

        assert!(result.is_ok());
        assert_eq!((login_attempt_id, code), result.unwrap());
    }
}
