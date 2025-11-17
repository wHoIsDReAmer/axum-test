use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::service::auth::traits::AuthService;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub(crate) async fn login(
    State(auth_service): State<Arc<dyn AuthService>>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    match auth_service
        .login(&request.username, &request.password)
        .await
    {
        Ok(token) => Json(LoginResponse { token }).into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}
