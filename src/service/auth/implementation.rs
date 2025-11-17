use super::{errors::AuthServiceError, traits::AuthService};
use crate::infrastructure::repository::auth::traits::AuthRepository;
use shaku::Component;
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = AuthService)]
pub(crate) struct AuthServiceImpl {
    #[shaku(inject)]
    repository: Arc<dyn AuthRepository>,
}

impl AuthService for AuthServiceImpl {
    fn login(&self, username: &str, password: &str) -> Result<String, AuthServiceError> {
        if username.trim().is_empty() || password.trim().is_empty() {
            return Err(AuthServiceError::InvalidCredentials);
        }

        match self.repository.verify_credentials(username, password) {
            Ok(true) => Ok(format!("token_{}", username)),
            Ok(false) => Err(AuthServiceError::InvalidCredentials),
            Err(e) => Err(AuthServiceError::DatabaseError(e.to_string())),
        }
    }
}
