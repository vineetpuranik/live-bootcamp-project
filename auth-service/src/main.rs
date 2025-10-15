use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::app_state::AppState;
use auth_service::{services::HashMapUserStore, Application};

#[tokio::main]
async fn main() {
    let user_store = HashMapUserStore {
        users: HashMap::new(),
    };
    let app_state = AppState {
        user_store: Arc::new(RwLock::new(user_store)),
    };

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run the app");
}
