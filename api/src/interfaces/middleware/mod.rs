use axum::{extract::{Request, State}, middleware::Next, response::Response};
use crate::interfaces::{app_state::AppState, auth::errors::AuthInterError};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request:  Request,
    next:         Next,
) -> Result<Response, AuthInterError> {
    let token = extract_token(&request)?;

    let claims = state
        .token_service
        .validate(&token)
        .map_err(|_| AuthInterError::InvalidToken)?;

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

fn extract_token(request: &Request) -> Result<String, AuthInterError> {
    // Cookie takes priority (browser flow)
    if let Some(cookie_header) = request.headers().get("Cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for part in cookie_str.split(';') {
                if let Some(value) = part.trim().strip_prefix("token=") {
                    return Ok(value.to_string());
                }
            }
        }
    }

    // Fall back to Authorization: Bearer (API clients / Kulala)
    let header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AuthInterError::MissingToken)?;

    header
        .strip_prefix("Bearer ")
        .map(|s| s.to_string())
        .ok_or(AuthInterError::MissingToken)
}
