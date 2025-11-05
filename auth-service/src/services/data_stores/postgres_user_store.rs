use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, Password, User,
};
use argon2::{
    password_hash::{Error as PasswordHashError, PasswordHash, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version,
};
use sqlx::{PgPool, Row};
use std::error::Error;
use tokio::task;

// PostgresUserStore persists account data inside the shared Postgres connection pool.
// The struct is lightweight—cloning the pool just bumps an Arc reference count.
pub struct PostgresUserStore {
    pub pool: PgPool,
}

impl PostgresUserStore {
    // Construct a store from an existing sqlx pool; the pool must already be configured with
    // the proper migrations because this layer assumes the `users` table exists.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    // Add a new user row, hashing the password and guarding against duplicate emails.
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Copy the database-bound fields up front so we can move `user` into the hashing task
        // without fighting the borrow checker.
        let email = user.email.as_ref().to_owned();
        let requires_2fa = user.requires_2fa;

        // Bail early if a row already exists for the supplied email address.
        // Using a simple 1-row query keeps contention low and gives us an explicit error path.
        let existing = sqlx::query_scalar::<_, i64>("SELECT 1 FROM users WHERE email = $1")
            .bind(&email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;
        if existing.is_some() {
            return Err(UserStoreError::UserAlreadyExists);
        }

        // calculate the hash of the user password
        // Hashing happens on the blocking pool so heavy crypto work never starves async executors.
        let password_hash = compute_password_hash(user.password.as_ref().to_owned())
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        // insert to users table
        // Any unique constraint violation is mapped back to domain errors; all other failures are
        // treated as operational surprises for the caller to retry/log.
        sqlx::query("INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)")
            .bind(&email)
            .bind(&password_hash)
            .bind(requires_2fa)
            .execute(&self.pool)
            .await
            .map_err(|error| match error {
                sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                    UserStoreError::UserAlreadyExists
                }
                _ => UserStoreError::UnexpectedError,
            })?;

        Ok(())
    }

    // Fetch a user by e-mail, returning a fully-populated domain object on success.
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        // Query the database for the stored credentials associated with the supplied email.
        let row =
            sqlx::query("SELECT email, password_hash, requires_2fa FROM users WHERE email = $1")
                .bind(email.as_ref())
                .fetch_optional(&self.pool)
                .await
                .map_err(|_| UserStoreError::UnexpectedError)?;

        let row = match row {
            Some(row) => row,
            None => return Err(UserStoreError::UserNotFound),
        };

        let email: String = row.get("email");
        let password_hash: String = row.get("password_hash");
        let requires_2fa: bool = row.get("requires_2fa");

        // Reconstruct the domain types before returning to keep the API consistent for callers.
        let email = Email::parse(email).map_err(|_| UserStoreError::UnexpectedError)?;
        // Persisted passwords are already hashed; we still wrap them in the domain `Password`
        // type so callers receive a consistent structure.
        let password =
            Password::parse(password_hash).map_err(|_| UserStoreError::UnexpectedError)?;

        Ok(User {
            email,
            password,
            requires_2fa,
        })
    }

    // Compare a supplied password against the stored hash, surfacing user-friendly errors.
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        // check if user exists. If user does not exist return UserStoreError::UserNotFound
        // if user exists then compute password hash of the passed input.
        // make sure it matches with the password hash stored in the db.
        // if there is no match then return UserStoreError::InvalidCredentials
        // Pull the persisted hash so we can compare it against the candidate password.
        let stored_hash =
            sqlx::query_scalar::<_, String>("SELECT password_hash FROM users WHERE email = $1")
                .bind(email.as_ref())
                .fetch_optional(&self.pool)
                .await
                .map_err(|_| UserStoreError::UnexpectedError)?;

        let stored_hash = match stored_hash {
            Some(hash) => hash,
            None => return Err(UserStoreError::UserNotFound),
        };

        let candidate = password.as_ref().to_owned();

        // Delegate the expensive verification to the helper so it can run on the blocking pool and
        // reuse the shared Argon2 configuration.
        match verify_password_hash(stored_hash, candidate).await {
            Ok(()) => Ok(()),
            Err(err) => {
                if err.downcast_ref::<PasswordHashError>().is_some() {
                    Err(UserStoreError::InvalidCredentials)
                } else {
                    // Anything other than a hash mismatch indicates infrastructure trouble (e.g.
                    // task cancellation), so report a generic unexpected error.
                    Err(UserStoreError::UnexpectedError)
                }
            }
        }
    }
}

// Helper function to verify if a given password matches an expected hash
// Hashing is a CPU intensive operation. To avoid blocking other async tasks, we will perform hashing on a separate thread pool
// tokio::task::spawn_blocking will be used to spin up a separate thread pool
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error>> {
    // spawn_blocking schedules this closure on Tokio's blocking thread pool so CPU-bound hashing
    // work does not starve async tasks running on the core executor.
    task::spawn_blocking(move || -> Result<(), argon2::password_hash::Error> {
        // Argon2 decodes the stored hash, replays the memory-hard key derivation, and compares the
        // derived hash with the stored value to confirm the password candidate.
        let expected_password_hash = PasswordHash::new(&expected_password_hash)?;
        Argon2::default().verify_password(password_candidate.as_bytes(), &expected_password_hash)
    })
    .await
    .map_err(|e| -> Box<dyn Error> { Box::new(e) })?
    .map_err(|e| -> Box<dyn Error> { Box::new(e) })
}

// Helper function to hash passwords before persisting them in the database.
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error>> {
    // spawn_blocking shifts the CPU-heavy password hashing work onto the blocking pool so async
    // tasks on the main executor stay responsive.
    let password_hash =
        task::spawn_blocking(move || -> Result<String, argon2::password_hash::Error> {
            // Argon2id (memory-hard and GPU-resistant) regenerates the salted hash parameters and
            // performs repeated mixing so attackers pay heavy CPU+RAM costs to brute-force guesses.
            let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
            let hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

            Ok(hash)
        })
        .await
        .map_err(|e| -> Box<dyn Error> { Box::new(e) })?
        .map_err(|e| -> Box<dyn Error> { Box::new(e) })?;

    Ok(password_hash)
}
