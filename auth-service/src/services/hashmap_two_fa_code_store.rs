use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use std::collections::HashMap;

#[derive(Default)]
pub struct HashMapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashMapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email);
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(value) => Ok(value.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};

    #[tokio::test]
    async fn add_and_get_code_should_succeed() {
        let mut store = HashMapTwoFACodeStore::default();

        let email = Email::parse("user@example.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .unwrap();

        let retrieved = store.get_code(&email).await.unwrap();
        assert_eq!(retrieved.0, login_attempt_id);
        assert_eq!(retrieved.1, code);
    }

    #[tokio::test]
    async fn get_code_should_return_error_for_missing_email() {
        let store = HashMapTwoFACodeStore::default();

        let email = Email::parse("user@example.com".to_owned()).unwrap();
        let result = store.get_code(&email).await;

        assert!(matches!(
            result,
            Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        ));
    }

    #[tokio::test]
    async fn remove_code_should_delete_existing_entry() {
        let mut store = HashMapTwoFACodeStore::default();

        let email = Email::parse("user@example.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        store
            .add_code(email.clone(), login_attempt_id, code)
            .await
            .unwrap();

        // Verify it's there
        assert!(store.get_code(&email).await.is_ok());

        // Remove it
        store.remove_code(&email).await.unwrap();

        // Now should not be found
        assert!(matches!(
            store.get_code(&email).await,
            Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        ));
    }

    #[tokio::test]
    async fn add_code_should_overwrite_existing_entry() {
        let mut store = HashMapTwoFACodeStore::default();

        let email = Email::parse("user@example.com".to_owned()).unwrap();

        let login_attempt_id1 = LoginAttemptId::default();
        let code1 = TwoFACode::default();

        store
            .add_code(email.clone(), login_attempt_id1, code1)
            .await
            .unwrap();

        let login_attempt_id2 = LoginAttemptId::default();
        let code2 = TwoFACode::default();

        store
            .add_code(email.clone(), login_attempt_id2.clone(), code2.clone())
            .await
            .unwrap();

        let retrieved = store.get_code(&email).await.unwrap();
        assert_eq!(retrieved.0, login_attempt_id2);
        assert_eq!(retrieved.1, code2);
    }
}
