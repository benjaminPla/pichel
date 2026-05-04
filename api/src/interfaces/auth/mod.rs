mod errors;

use crate::{
    domain::auth::Claims,
    interfaces::{app_state::AppState, auth::errors::AuthInterError},
};
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};

pub struct AuthUser(Claims);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AuthInterError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthInterError::MissingToken)?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or(AuthInterError::MissingToken)?;

        let claims = state
            .token_service
            .validate(token)
            .map_err(|_| AuthInterError::InvalidToken)?;

        Ok(AuthUser(claims))
    }
}
