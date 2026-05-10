mod dto;

use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use crate::{
    application::auth::authenticate::{AuthenticateInput, AuthenticateUseCase},
    interfaces::{
        app_state::AppState,
        auth::{
            errors::AuthInterError,
            handlers::authenticate::dto::{AuthenticateRequestBody, AuthenticateResponse},
        },
    },
};

pub async fn authenticate(
    State(app_state): State<AppState>,
    Json(body):       Json<AuthenticateRequestBody>,
) -> Result<impl IntoResponse, AuthInterError> {
    let token = AuthenticateUseCase::new(
        app_state.hasher_service,
        app_state.token_service.clone(),
        app_state.user_repo,
    )
    .execute(AuthenticateInput {
        email:    body.email,
        password: body.password,
    })
    .await?;

    let cookie = format!(
        "token={}; HttpOnly{}; SameSite=Strict; Path=/; Max-Age=86400",
        token,
        if app_state.cookie_secure { "; Secure" } else { "" },
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).map_err(|e| AuthInterError::Internal(e.to_string()))?,
    );

    Ok((StatusCode::OK, headers, Json(AuthenticateResponse { token })))
}
