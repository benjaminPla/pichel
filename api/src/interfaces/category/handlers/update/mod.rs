pub mod dto;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use crate::{
    application::category::update::{UpdateCategoryInput, UpdateCategoryUseCase},
    interfaces::{
        app_state::AppState,
        category::{
            errors::CategoryInterError,
            handlers::update::dto::{CategoryUpdateRequestBody, CategoryUpdateResponse},
        },
    },
};
use uuid::Uuid;

pub async fn update(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
    Json(body):       Json<CategoryUpdateRequestBody>,
) -> Result<impl IntoResponse, CategoryInterError> {
    let category = UpdateCategoryUseCase::new(app_state.category_repo)
        .execute(UpdateCategoryInput { id, name: body.name })
        .await?;
    Ok((StatusCode::OK, Json(CategoryUpdateResponse::from(category))))
}
