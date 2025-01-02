use crate::repository::auth::traits::AuthRepository;

use super::errors::AuthServiceError;

struct AuthService {
    repository: Box<dyn AuthRepository>,
}

impl AuthService {
    pub(crate) fn new(repository: Box<dyn AuthRepository>) -> Self {
        Self { repository }
    }
}

impl super::traits::AuthService for AuthService {
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
