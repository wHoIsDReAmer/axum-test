#[derive(Debug)]
pub(crate) enum AuthServiceError {
    InvalidCredentials,
    DatabaseError(String),
}

impl std::fmt::Display for AuthServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
