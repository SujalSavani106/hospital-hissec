use std::fmt;

/// Application-level errors.
#[derive(Debug)]
pub enum AppError {
    /// PEP denied the request.
    PolicyDenied(String),
    /// The requested resource was not found.
    NotFound(String),
    /// Invalid request parameters.
    BadRequest(String),
    /// Authentication failure.
    Unauthorized(String),
    /// Internal system error.
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::PolicyDenied(m)  => write!(f, "DENY: {}", m),
            AppError::NotFound(m)      => write!(f, "Not Found: {}", m),
            AppError::BadRequest(m)    => write!(f, "Bad Request: {}", m),
            AppError::Unauthorized(m)  => write!(f, "Unauthorized: {}", m),
            AppError::Internal(m)      => write!(f, "Internal Error: {}", m),
        }
    }
}
