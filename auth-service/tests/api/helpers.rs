use auth_service::app_state::{BannedTokenStoreType, EmailClientType, TwoFACodeStoreType};
use auth_service::services::{HashMapTwoFACodeStore, HashsetBannedTokenStore, MockEmailClient};
use auth_service::{
    app_state::AppState, domain::UserStore, services::HashMapUserStore, utils::constants::test,
    Application,
};
use reqwest::cookie::Jar;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType, // New!
    pub email_client: EmailClientType,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store: Box<dyn UserStore + Send + Sync> = Box::new(HashMapUserStore {
            users: HashMap::new(),
        });

        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let two_fa_code_store = Arc::new(RwLock::new(HashMapTwoFACodeStore::default()));
        let email_client = Arc::new(RwLock::new(MockEmailClient));

        let app_state = AppState::new(
            Arc::new(RwLock::new(user_store)),
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            email_client.clone(),
        );

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build the app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service is a separate async task
        // This will make sure that we do not block the main test thread

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        // Create a reqwest client backed by the shared cookie jar so tests can set cookies
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .expect("Failed to build http client");

        // Create a new TestApp instance and return it
        Self {
            address,
            cookie_jar,
            banned_token_store: banned_token_store,
            two_fa_code_store: two_fa_code_store,
            email_client: email_client,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute a request")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute a request")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute a request")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute a request")
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute a request")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
