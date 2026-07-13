use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension};
use crate::{
    application::product::delete::{DeleteProductInput, DeleteProductUseCase},
    domain::auth::Claims,
    interfaces::{app_state::AppState, product::errors::ProductInterError},
};
use uuid::Uuid;

pub async fn delete(
    State(app_state):  State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id):          Path<Uuid>,
) -> Result<impl IntoResponse, ProductInterError> {
    DeleteProductUseCase::new(app_state.product_repo)
        .execute(DeleteProductInput { id, updated_by: claims.get_sub().value() })
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
