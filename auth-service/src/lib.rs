use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tower_http::services::ServeDir;

pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

use crate::routes::*;
use app_state::AppState;

//This struct encapsulates our application related logic
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field.
    // this makes it possible to access address in tests
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from main.rs here
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

pub mod app_state {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use crate::domain::UserStore;

    // we will use a type alias for representing Arc<RwLock<Box<dyn UserStore>>>
    // Wrapping the user store in an Arc allows shared ownership of the underlying store across threads.
    // Calling clone on an Arc produces a new Arc instance which points to the same allocation on the heap as source Arc.
    // Instead of copying the reference data, the reference count is incremented.

    // Arc only provides an immutable reference to the underlying data (user store in our case)
    // It mutation were allowed, it would introduce possibility of data races and inconsistencies between threads.
    // However, route handlers need mutable access to user store to add / remove users from user store.
    // To achieve this we need a synchronization primitive and we will be using the RwLock provided by tokio.

    // In Summary, by wrapping the user store in tokio's RwLock smart pointer, the user store can be safely mutated across threads.
    // By wrapping RwLock<Box<dyn UserStore>> in an Arc smart pointer, the underlying data can be shared across threads while maintaining a single source of truth.
    pub type UserStoreType = Arc<RwLock<Box<dyn UserStore + Send + Sync>>>;

    #[derive(Clone)]
    // AppState derives the Clone trait
    // Axum clones th application state before passing it into route handlers
    // This ensures that each request has a consistent snapshot of the state without causing race or inconsistencies.
    pub struct AppState {
        pub user_store: UserStoreType,
    }

    impl AppState {
        pub fn new(user_store: UserStoreType) -> Self {
            Self { user_store }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid Credentials"),
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED, "IncorrectCredentials")
            }
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}
