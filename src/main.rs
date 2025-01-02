mod repository;
mod service;
mod infrastructure;
mod config;

use std::path::PathBuf;

use config::database_config::DatabaseConfig;
use infrastructure::postgres::create_connection;
use repository::auth::{implementation::AuthRepositoryImpl, traits::AuthRepository};
use sqlx::PgPool;
use teloc::{Resolver, ServiceProvider};
use tokio;
use anyhow::Result;

fn setup_logger() {
    quicklog::init!();
}

fn load_database_config() -> Result<DatabaseConfig> {
    let path = PathBuf::from("config/database.toml");
    DatabaseConfig::try_from(path)
}

#[tokio::main]
async fn main() {
    setup_logger();

    let config = load_database_config().expect("Failed to load database config");
    let db = create_connection(&config).await.expect("Failed to connect to database");

//     let port = 3000;
//     let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

//     let app = Router::new()
//         .route("/", get(hello_world));
//     axum::serve::serve(listener, app)
//         .await
//         .expect("Failed to serve");
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
