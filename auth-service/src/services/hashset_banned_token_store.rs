use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    pub bannedTokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn store_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.bannedTokens.insert(token);
        Ok(())
    }

    async fn check_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let result = self.bannedTokens.contains(token);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_token() {
        let mut test_store = HashsetBannedTokenStore::default();
        let test_token = "test_token".to_owned();
        let test_result  = test_store.store_token(test_token.clone()).await;
        assert!(test_result.is_ok());
    }

    #[tokio::test]
    async fn test_check_token() {
        let mut test_store = HashsetBannedTokenStore::default();
        let test_token = "test_token".to_owned();
        let test_result  = test_store.store_token(test_token.clone()).await;
        assert!(test_result.is_ok());

        let check_result = test_store.check_token(&test_token).await;
        assert!(check_result.is_ok());
    }
}
