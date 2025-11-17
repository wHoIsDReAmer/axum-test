use std::sync::Arc;

use async_trait::async_trait;
use shaku::Component;
use sqlx::PgPool;

use super::{errors::AuthRepositoryError, traits::AuthRepository};

#[derive(Component)]
#[shaku(interface = AuthRepository)]
pub(crate) struct AuthRepositoryImpl {
    db: Arc<PgPool>,
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn verify_credentials(
        &self,
        username: &str,
        password: &str,
    ) -> Result<bool, AuthRepositoryError> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 AND password = $2)",
        )
        .bind(username)
        .bind(password)
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| AuthRepositoryError::DatabaseError(e.to_string()))?;

        Ok(exists)
    }
}
