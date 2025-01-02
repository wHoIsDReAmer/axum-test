mod repository;
mod service;

use std::collections::HashMap;

use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use tokio;
use tokio::net::TcpListener;

fn setup_logger() {
    quicklog::init!();
}

#[tokio::main]
async fn main() {
    setup_logger();

    let port = 3000;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    let app = Router::new()
        .route("/", get(hello_world));
    axum::serve::serve(listener, app)
        .await
        .expect("Failed to serve");
}

async fn hello_world(
    name: Query<HashMap<String, String>>,
) -> axum::response::Response<String> {
    let response_text = {
        let name = name.0.get("name");
        match name {
            Some(name) if name.trim().is_empty() => "Hello, Anonymous!".to_string(),
            Some(name) => {
                let sanitized = name.trim().to_string();
                format!("Hello, {}!", sanitized)
            },
            None => "Hello, World!".to_string(),
        }
    };

    axum::response::Response::builder()
        .header("Content-Type", "text/plain")
        .body(response_text)
        .unwrap()
}
