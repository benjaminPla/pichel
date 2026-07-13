use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse};
use crate::{
    application::user::delete::{DeleteUserInput, DeleteUserUseCase},
    interfaces::{app_state::AppState, user::errors::UserInterError},
};
use uuid::Uuid;

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
) -> Result<impl IntoResponse, UserInterError> {
    DeleteUserUseCase::new(app_state.user_repo)
        .execute(DeleteUserInput { id })
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
