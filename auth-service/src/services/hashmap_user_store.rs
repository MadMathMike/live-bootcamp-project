use std::collections::HashMap;

use crate::domain::{Email, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        if !self.users.contains_key(email) {
            Err(UserStoreError::UserNotFound)
        } else {
            Ok(self.users[email].clone())
        }
    }

    async fn validate_user(&self, email: &Email, password: &str) -> Result<(), UserStoreError> {
        if !self.users.contains_key(email) {
            return Err(UserStoreError::UserNotFound)
        } 
        
        let user = &self.users[email];

        if password != user.password {
            Err(UserStoreError::InvalidCredentials)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Email;

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();

        // First time user is added should return Ok(())
        let email = Email::parse("blah@".to_owned()).unwrap();
        let user = User::new(email, "blah".to_owned(), false);
        let result = store.add_user(user.clone()).await;
        assert!(result.is_ok());

        // Second time user is added should return Err(UserStoreError::UserAlreadyExists)
        let result = store.add_user(user).await;
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert_eq!(UserStoreError::UserAlreadyExists, err);
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();

        let email = Email::parse("blah@".to_owned()).unwrap();
        let user = User::new(email.clone(), "blah".to_owned(), false);

        let user_result = store.get_user(&email).await;
        assert!(user_result.is_err());
        assert_eq!(UserStoreError::UserNotFound, user_result.err().unwrap());

        let result = store.add_user(user.clone()).await;
        assert!(result.is_ok());

        let user_result = store.get_user(&email).await;
        assert!(result.is_ok());

        let returned_user = user_result.ok().unwrap();
        assert_eq!(email.as_ref(), returned_user.email.as_ref());
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();

        let email = Email::parse("blah@".to_owned()).unwrap();
        let password = "blah".to_owned();
        
        let validation_result = store.validate_user(&email, &password).await;
        assert!(validation_result.is_err());
        let err = validation_result.err().unwrap();
        assert_eq!(UserStoreError::UserNotFound, err);
        
        let user = User::new(email.clone(), password, false);
        let result = store.add_user(user.clone()).await;
        assert!(result.is_ok());

        let validation_result = store.validate_user(&email, "wrong password").await;
        assert!(validation_result.is_err());
        let err = validation_result.err().unwrap();
        assert_eq!(UserStoreError::InvalidCredentials, err);

        let validation_result = store.validate_user(&email, &user.password).await;
        assert!(validation_result.is_ok());
    }
}
