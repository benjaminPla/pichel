pub mod dto;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::{
    application::category::create::{CreateCategoryInput, CreateCategoryUseCase},
    interfaces::{
        app_state::AppState,
        category::{
            errors::CategoryInterError,
            handlers::create::dto::{CategoryCreateRequestBody, CategoryCreateResponse},
        },
    },
};

pub async fn create(
    State(app_state): State<AppState>,
    Json(body):       Json<CategoryCreateRequestBody>,
) -> Result<impl IntoResponse, CategoryInterError> {
    let category = CreateCategoryUseCase::new(app_state.category_repo)
        .execute(CreateCategoryInput { name: body.name })
        .await?;
    Ok((StatusCode::CREATED, Json(CategoryCreateResponse::from(category))))
}
