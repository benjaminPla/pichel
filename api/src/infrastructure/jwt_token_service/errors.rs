use crate::domain::auth::ports::TokenServiceError;

impl From<jsonwebtoken::errors::Error> for TokenServiceError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::Internal(e.to_string())
    }
}

impl From<uuid::Error> for TokenServiceError {
    fn from(e: uuid::Error) -> Self {
        Self::Internal(e.to_string())
    }
}
