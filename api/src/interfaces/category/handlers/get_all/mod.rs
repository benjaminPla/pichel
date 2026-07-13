pub mod dto;

use crate::{
    application::category::get_all::GetAllCategoriesUseCase,
    interfaces::{
        app_state::AppState,
        category::{errors::CategoryInterError, handlers::get_all::dto::{CategoryGetAllItem, CategoryGetAllResponse}},
    },
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn get_all(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, CategoryInterError> {
    let categories = GetAllCategoriesUseCase::new(app_state.category_repo)
        .execute()
        .await?;
    let categories = categories.into_iter().map(CategoryGetAllItem::from).collect::<Vec<_>>();
    Ok((StatusCode::OK, Json(CategoryGetAllResponse { categories })))
}
