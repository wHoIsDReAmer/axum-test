use sqlx::PgPool;
use teloc::Dependency;

use super::{errors::AuthRepositoryError, traits::AuthRepository};

#[derive(Dependency)]
pub (crate) struct AuthRepositoryImpl {
    db: PgPool,
}

impl AuthRepository for AuthRepositoryImpl {
    fn verify_credentials(&self, username: &str, password: &str) -> Result<bool, AuthRepositoryError> {
        Ok(true)
    }
}
