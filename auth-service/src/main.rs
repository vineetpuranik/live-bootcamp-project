use auth_service::services::HashsetBannedTokenStore;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::app_state::AppState;
use auth_service::domain::UserStore;
use auth_service::{services::HashMapUserStore, utils::constants::prod, Application};

#[tokio::main]
async fn main() {
    let user_store: Box<dyn UserStore + Send + Sync> = Box::new(HashMapUserStore {
        users: HashMap::new(),
    });

    let banned_token_store = HashsetBannedTokenStore {
        banned_tokens: HashSet::new(),
    };

    let user_store = Arc::new(RwLock::new(user_store));
    let banned_token_store = Arc::new(RwLock::new(banned_token_store));

    let app_state = AppState::new(user_store, banned_token_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run the app");
}
