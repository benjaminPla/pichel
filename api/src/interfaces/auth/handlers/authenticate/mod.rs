mod dto;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::{
    application::auth::commands::authenticate::{AuthenticateCommand, AuthenticateHandler},
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
    let token = AuthenticateHandler::new(
        app_state.hasher_service,
        app_state.token_service,
        app_state.user_repo,
    )
    .execute(AuthenticateCommand {
        email:    body.email,
        password: body.password,
    })
    .await?;
    Ok((StatusCode::OK, Json(AuthenticateResponse { token })))
}
