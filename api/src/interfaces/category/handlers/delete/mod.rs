use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse};
use crate::{
    application::category::delete::{DeleteCategoryInput, DeleteCategoryUseCase},
    interfaces::{app_state::AppState, category::errors::CategoryInterError},
};
use uuid::Uuid;

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
) -> Result<impl IntoResponse, CategoryInterError> {
    DeleteCategoryUseCase::new(app_state.category_repo)
        .execute(DeleteCategoryInput { id })
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
