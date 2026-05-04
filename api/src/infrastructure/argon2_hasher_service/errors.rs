use argon2::password_hash;
use crate::domain::hasher::HasherServiceError;

impl From<password_hash::Error> for HasherServiceError {
    fn from(e: password_hash::Error) -> Self {
        HasherServiceError::Hash(e.to_string())
    }
}

impl From<tokio::task::JoinError> for HasherServiceError {
    fn from(e: tokio::task::JoinError) -> Self {
        HasherServiceError::TokioTask(e.to_string())
    }
}
