use shaku::Interface;

use super::errors::AuthServiceError;

pub(crate) trait AuthService: Interface {
    fn login(&self, username: &str, password: &str) -> Result<String, AuthServiceError>;
}
