use super::errors::AuthServiceError;

pub(crate) trait AuthService {
    fn login(&self, username: &str, password: &str) -> Result<String, AuthServiceError>;
}
