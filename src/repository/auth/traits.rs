use teloc::Dependency;

use super::errors::AuthRepositoryError;

pub(crate) trait AuthRepository {
    fn verify_credentials(&self, username: &str, password: &str) -> Result<bool, AuthRepositoryError>;
}
