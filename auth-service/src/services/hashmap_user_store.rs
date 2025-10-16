use std::collections::HashMap;

use crate::domain::{User, UserStoreError};
use crate::domain::UserStore;

// deriving Default trait ensures we can create new instances of HashMapUserStore that contain an empty HashMap
#[derive(Default)]
pub struct HashMapUserStore {
    pub users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // If user already exists, return a UserAlreadyExists error
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        // insert the user into our hashmap and return ok
        self.users.insert(String::from(&user.email), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        // This function should return a `Result` type containing either a
        // `User` object or a `UserStoreError::UserNotFound`.
        if let Some(user) = self.users.get(email) {
            Ok(user)
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email) {
            // check if password matches
            if !(user.password == password) {
                return Err(UserStoreError::InvalidCredentials);
            } else {
                Ok(())
            }
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        // create a user_store_map instance
        let mut user_store_map: Box<dyn UserStore + Send + Sync> = Box::new(HashMapUserStore {
            users: HashMap::new(),
        });

        // create a user instance to be added to the store
        let user_to_add = User {
            email: "mytestemail@test.com".to_string(),
            password: "mypass".to_string(),
            requires_2fa: false,
        };

        // add user to the store
        let _ = user_store_map.add_user(user_to_add.clone()).await;

        // assert that 1 user is present in the user store map
        // assert_eq!(user_store_map.users.len(), 1);

        // assert that we get an UserAlreadyExists error on attempting to add the same user.
        assert_eq!(
            user_store_map.add_user(user_to_add).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        // create a user_store_map instance
        let mut user_store_map: Box<dyn UserStore + Send + Sync> = Box::new(HashMapUserStore {
            users: HashMap::new(),
        });

        // create a user instance to be added to the store
        let user_to_add = User {
            email: "mytestemail@test.com".to_string(),
            password: "mypass".to_string(),
            requires_2fa: false,
        };

        // assert UserNotFound returned by get_user since we have not yet added user to the store
        assert_eq!(
            user_store_map.get_user(&user_to_add.email).await,
            Err(UserStoreError::UserNotFound)
        );

        // add the user to the store
        let _ = user_store_map.add_user(user_to_add.clone()).await;

        // assert that we are able to return the newly added user by calling get_user
        assert_eq!(
            user_store_map.get_user(&user_to_add.email).await,
            Ok(&user_to_add)
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        // create a user_store_map instance as HashMapUserStore
        let mut user_store_map = HashMapUserStore {
            users: HashMap::new(),
        };

        let test_email = "mytestemail@test.com";
        let test_password = "mypass";
        let test_incorrect_password = "wrongpass";

        // Assert we get UserNotFound if user email is not present in user store map
        assert_eq!(
            user_store_map.validate_user(test_email, test_password).await,
            Err(UserStoreError::UserNotFound)
        );

        user_store_map.users.insert(
            test_email.to_string(),
            User {
                email: test_email.to_string(),
                password: test_password.to_string(),
                requires_2fa: false,
            },
        );

        // Assert validate user returns () with valid email and password
        assert_eq!(
            user_store_map.validate_user(test_email, test_password).await,
            Ok(())
        );

       // Assert validate user returns InvalidCredentials when password is incorrect
        assert_eq!(
            user_store_map.validate_user(test_email, test_incorrect_password).await,
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
