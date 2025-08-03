use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashSetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashSetBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        if !self.tokens.contains(&token) {
            self.tokens.insert(token);
        }

        Ok(())
    }

    async fn is_banned(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.tokens.contains(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashSetBannedTokenStore::default();

        let add_token_result = store.add_token(String::from("anything")).await;

        assert!(add_token_result.is_ok());
    }

    #[tokio::test]
    async fn test_is_banned() {
        let mut store = HashSetBannedTokenStore::default();

        store.add_token(String::from("anything")).await.unwrap();

        let is_banned_result = store.is_banned("anything else").await;
        assert!(is_banned_result.is_ok());
        
        let banned = is_banned_result.unwrap();
        assert!(!banned);
    }
}
    