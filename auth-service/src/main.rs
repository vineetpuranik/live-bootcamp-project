use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::app_state::AppState;
use auth_service::domain::UserStore;
use auth_service::{services::HashMapUserStore, Application};

#[tokio::main]
async fn main() {
    let user_store: Box<dyn UserStore + Send + Sync> = Box::new(HashMapUserStore {
        users: HashMap::new(),
    });
    let user_store = Arc::new(RwLock::new(user_store));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run the app");
}
