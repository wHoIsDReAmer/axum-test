use std::sync::Arc;

use shaku::Component;
use sqlx::PgPool;

use super::{errors::AuthRepositoryError, traits::AuthRepository};

#[derive(Component)]
#[shaku(interface = AuthRepository)]
pub(crate) struct AuthRepositoryImpl {
    db: Arc<PgPool>,
}

impl AuthRepository for AuthRepositoryImpl {
    fn verify_credentials(
        &self,
        username: &str,
        password: &str,
    ) -> Result<bool, AuthRepositoryError> {
        Ok(true)
    }
}
