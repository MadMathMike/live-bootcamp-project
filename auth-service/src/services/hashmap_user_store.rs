use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.

    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();

        // First time user is added should return Ok(())
        let user = User::new();
        let result = store.add_user(user);
        assert!(result.is_ok());

        // Second time user is added should return Err(UserStoreError::UserAlreadyExists)
        let user = User::new();
        let result = store.add_user(user);
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert_eq!(UserStoreError::UserAlreadyExists, err);
    }

    #[tokio::test]
    async fn test_get_user() {
        todo!()
    }

    #[tokio::test]
    async fn test_validate_user() {
        todo!()
    }
}
