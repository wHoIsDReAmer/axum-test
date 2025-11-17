use shaku::Interface;

use super::errors::AuthRepositoryError;

pub(crate) trait AuthRepository: Interface {
    fn verify_credentials(
        &self,
        username: &str,
        password: &str,
    ) -> Result<bool, AuthRepositoryError>;
}
