mod config;
mod di;
mod handler;
mod infrastructure;
mod service;

mod di_provider;

use std::path::PathBuf;

use anyhow::Result;
use axum::{routing::post, Router};
use config::database_config::DatabaseConfig;
use di::{auth::AuthModule, db::DatabaseModule};
use di_provider::{setup_auth_service, setup_pool};
use handler::auth;
use infrastructure::postgres::{
    PostgresConnection, PostgresConnectionImpl, PostgresConnectionImplParameters,
};
use infrastructure::repository::auth::implementation::{
    AuthRepositoryImpl, AuthRepositoryImplParameters,
};
use service::auth::{implementation::AuthServiceImpl, traits::AuthService};
use tokio::{self, net::TcpListener};

fn setup_logger() {
    quicklog::init!();
}

fn load_database_config() -> Result<DatabaseConfig> {
    let path = PathBuf::from("config/database.toml");
    DatabaseConfig::try_from(path)
}

async fn setup_app() -> Router {
    let config = load_database_config().expect("Failed to load database config");

    let pool = setup_pool(config).await;
    let auth_service = setup_auth_service(pool).await;

    

    Router::new()
        .route("/", post(auth::login))
        .with_state(auth_service)
}

#[tokio::main]
async fn main() {
    setup_logger();

    let port = 3000;

    let app = setup_app().await;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve::serve(listener, app)
        .await
        .expect("Failed to serve");
}

// async fn hello_world(
//     name: Query<HashMap<String, String>>,
// ) -> axum::response::Response<String> {
//     let response_text = {
//         let name = name.0.get("name");
//         match name {
//             Some(name) if name.trim().is_empty() => "Hello, Anonymous!".to_string(),
//             Some(name) => {
//                 let sanitized = name.trim().to_string();
//                 format!("Hello, {}!", sanitized)
//             },
//             None => "Hello, World!".to_string(),
//         }
//     };

//     axum::response::Response::builder()
//         .header("Content-Type", "text/plain")
//         .body(response_text)
//         .unwrap()
// }
