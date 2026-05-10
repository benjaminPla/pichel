pub mod dto;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use crate::{
    application::user::delete::{DeleteUserInput, DeleteUserUseCase},
    interfaces::{
        app_state::AppState,
        user::{errors::UserInterError, handlers::delete::dto::UserDeleteResponse},
    },
};
use uuid::Uuid;

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = DeleteUserUseCase::new(app_state.user_repo)
        .execute(DeleteUserInput { id })
        .await?;
    Ok((StatusCode::GONE, Json(UserDeleteResponse::from(user))))
}
