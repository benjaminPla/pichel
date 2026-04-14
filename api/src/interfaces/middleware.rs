// ── Auth Middleware ───────────────────────────────────────────────────────────
//
// The DDD payoff: this extractor speaks pure domain language.
// Handlers that need auth declare `user: AuthenticatedUser` as a parameter —
// they receive a UserId with zero JWT knowledge leaking through.

use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{domain::auth::user::UserId, interfaces::{errors::AppError, state::AppState}};

pub struct AuthenticatedUser(pub UserId);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        let user_id = state
            .token_service
            .validate(token)
            .map_err(|_| AppError::Unauthorized)?;

        Ok(AuthenticatedUser(user_id))
    }
}
