use std::collections::HashMap;

use crate::domain::UserStore;
use crate::domain::{Email, Password, User, UserStoreError};

// deriving Default trait ensures we can create new instances of HashMapUserStore that contain an empty HashMap
#[derive(Default)]
pub struct HashMapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // If user already exists, return a UserAlreadyExists error
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        // insert the user into our hashmap and return ok
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        // This function should return a `Result` type containing either a
        // `User` object or a `UserStoreError::UserNotFound`.
        if let Some(user) = self.users.get(email) {
            Ok(user.clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email) {
            // check if password matches
            if !(user.password.eq(password)) {
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

        // create a user instance to be added to the storeE)
        let user_to_add = User {
            email: Email::parse("mytestemail@test.com".to_owned()).unwrap(),
            password: Password::parse("Password@12345".to_owned()).unwrap(),
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
            email: Email::parse("mytestemail@test.com".to_owned()).unwrap(),
            password: Password::parse("Password@12345".to_owned()).unwrap(),
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
            Ok(user_to_add)
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        // create a user_store_map instance as HashMapUserStore
        let mut user_store_map = HashMapUserStore {
            users: HashMap::new(),
        };

        let test_email: Email = Email::parse("mytestemail@test.com".to_owned()).unwrap();
        let test_password: Password = Password::parse("Password@12345".to_owned()).unwrap();
        let test_incorrect_password: Password = Password::parse("wrongpass".to_owned()).unwrap();

        // Assert we get UserNotFound if user email is not present in user store map
        assert_eq!(
            user_store_map
                .validate_user(&test_email, &test_password)
                .await,
            Err(UserStoreError::UserNotFound)
        );

        user_store_map.users.insert(
            test_email.clone(),
            User {
                email: test_email.clone(),
                password: test_password.clone(),
                requires_2fa: false,
            },
        );

        // Assert validate user returns () with valid email and password
        assert_eq!(
            user_store_map
                .validate_user(&test_email, &test_password)
                .await,
            Ok(())
        );

        // Assert validate user returns InvalidCredentials when password is incorrect
        assert_eq!(
            user_store_map
                .validate_user(&test_email, &test_incorrect_password)
                .await,
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
