mod errors;

use axum::{extract::{Request, State}, middleware::Next, response::Response};
use crate::interfaces::{app_state::AppState, auth::errors::AuthInterError};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request:  Request,
    next:         Next,
) -> Result<Response, AuthInterError> {
    let header = request
        .headers()
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

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
