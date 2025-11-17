use async_trait::async_trait;
use shaku::Interface;

use super::errors::AuthServiceError;

#[async_trait]
pub(crate) trait AuthService: Interface {
    async fn login(&self, username: &str, password: &str) -> Result<String, AuthServiceError>;
}
