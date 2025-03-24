use std::sync::Arc;
use shaku::Component;
use crate::infrastructure::repository::auth::traits::AuthRepository;
use super::{errors::AuthServiceError, traits::AuthService};

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
            Ok(true) => {
                Ok(format!("token_{}", username))
            }
            Ok(false) => Err(AuthServiceError::InvalidCredentials),
            Err(e) => Err(AuthServiceError::DatabaseError(e.to_string())),
        }
    }
}
