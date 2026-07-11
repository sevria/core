#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Resource already exists")]
    AlreadyExists,

    #[error("Authentication failed")]
    Authentication,

    #[error("Authorization failed")]
    Authorization,

    #[error("Internal server error")]
    Internal,

    #[error("Resource not found")]
    NotFound,

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Validation failed")]
    Validation(#[from] garde::Report),
}
