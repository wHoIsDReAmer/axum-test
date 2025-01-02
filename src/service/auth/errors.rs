#[derive(Debug)]
pub(crate) enum AuthServiceError {
    InvalidCredentials,
    DatabaseError(String),
}
