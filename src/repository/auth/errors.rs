#[derive(Debug)]
pub(crate) enum AuthRepositoryError {
    DatabaseError(String),
}

impl std::error::Error for AuthRepositoryError {}

impl std::fmt::Display for AuthRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
