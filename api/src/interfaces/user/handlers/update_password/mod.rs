pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    application::user::update_password::{UpdateUserPasswordInput, UpdateUserPasswordUseCase},
    interfaces::{
        app_state::AppState,
        user::{
            errors::UserInterError,
            handlers::update_password::dto::{
                UserUpdatePasswordRequestBody, UserUpdatePasswordResponse,
            },
        },
    },
};
use uuid::Uuid;

pub async fn update_password(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
    Json(body):       Json<UserUpdatePasswordRequestBody>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UpdateUserPasswordUseCase::new(app_state.hasher_service, app_state.user_repo)
        .execute(UpdateUserPasswordInput { id, password: body.password })
        .await?;
    Ok((StatusCode::OK, Json(UserUpdatePasswordResponse::from(user))))
}
