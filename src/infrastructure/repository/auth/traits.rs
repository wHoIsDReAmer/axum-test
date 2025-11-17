use async_trait::async_trait;
use shaku::Interface;

use super::errors::AuthRepositoryError;

#[async_trait]
pub(crate) trait AuthRepository: Interface {
    async fn verify_credentials(
        &self,
        username: &str,
        password: &str,
    ) -> Result<bool, AuthRepositoryError>;
}
