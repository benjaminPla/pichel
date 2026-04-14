use axum::{extract::State, Json};

use crate::interfaces::{
    auth::dto::{LoginRequest, RegisterRequest, TokenResponse},
    errors::AppError,
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let token = state.register_uc.execute(body.email, body.password).await?;
    Ok(Json(TokenResponse { token }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let token = state.login_uc.execute(body.email, body.password).await?;
    Ok(Json(TokenResponse { token }))
}
