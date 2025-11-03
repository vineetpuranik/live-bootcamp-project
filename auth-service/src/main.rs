use auth_service::services::{HashMapTwoFACodeStore, HashsetBannedTokenStore, MockEmailClient};
use auth_service::utils::DATABASE_URL;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::app_state::AppState;
use auth_service::domain::UserStore;
use auth_service::{services::HashMapUserStore, utils::constants::prod, Application};
use auth_service::get_postgres_pool;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;

    let user_store: Box<dyn UserStore + Send + Sync> = Box::new(HashMapUserStore {
        users: HashMap::new(),
    });

    let banned_token_store = HashsetBannedTokenStore {
        banned_tokens: HashSet::new(),
    };

    let two_fa_code_store = HashMapTwoFACodeStore {
        codes: HashMap::new(),
    };

    let email_client = MockEmailClient;

    let user_store = Arc::new(RwLock::new(user_store));
    let banned_token_store = Arc::new(RwLock::new(banned_token_store));
    let two_fa_code_store = Arc::new(RwLock::new(two_fa_code_store));
    let email_client = Arc::new(RwLock::new(email_client));

    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run the app");
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database! 
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}