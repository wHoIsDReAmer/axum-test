#[derive(Debug)]
pub(crate) enum AuthServiceError {
    InvalidCredentials,
    DatabaseError(String),
}

impl std::fmt::Display for AuthServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthServiceError::InvalidCredentials => write!(f, "Invalid username or password"),
            AuthServiceError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}
