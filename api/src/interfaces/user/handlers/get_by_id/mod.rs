pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    application::user::get_by_id::{GetUserByIdInput, GetUserByIdUseCase},
    interfaces::{
        app_state::AppState,
        user::{errors::UserInterError, handlers::get_by_id::dto::UserGetByIdResponse},
    },
};
use uuid::Uuid;

pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = GetUserByIdUseCase::new(app_state.user_repo)
        .execute(GetUserByIdInput { id })
        .await?;
    Ok((StatusCode::OK, Json(UserGetByIdResponse::from(user))))
}
