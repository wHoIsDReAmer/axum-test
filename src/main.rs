mod service;
mod infrastructure;
mod config;
mod handler;
mod di;

use std::{path::PathBuf, sync::Arc};

use axum::{routing::post, Router};
use config::database_config::DatabaseConfig;
use di::{auth::AuthModule, db::DatabaseModule};
use handler::auth;
use infrastructure::postgres::{PostgresConnection, PostgresConnectionImpl, PostgresConnectionImplParameters};
use infrastructure::repository::auth::implementation::{AuthRepositoryImpl, AuthRepositoryImplParameters};
use service::auth::{implementation::AuthServiceImpl, traits::AuthService};
use shaku::HasComponent;
use tokio::{self, net::TcpListener};
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
    
    let db_module = DatabaseModule::builder()
        .with_component_parameters::<PostgresConnectionImpl>(PostgresConnectionImplParameters {
            config: Arc::new(config),
            pool: tokio::sync::OnceCell::new(),
        })
        .build();
    let conn: &dyn PostgresConnection = db_module.resolve_ref();
    let pool = conn.get_pool().await.expect("Failed to get database pool");
    
    let auth_module = AuthModule::builder()
        .with_component_parameters::<AuthRepositoryImpl>(AuthRepositoryImplParameters {
            db: pool,
        })
        .build();
    let auth_service: Arc<dyn AuthService> = auth_module.resolve();

    let port = 3000;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    let app = Router::new()
        .route("/", post(auth::login))
        .with_state(auth_service);
    
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
