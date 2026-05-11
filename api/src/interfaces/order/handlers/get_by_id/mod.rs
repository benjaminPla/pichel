pub mod dto;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;
use crate::{
    application::order::get_by_id::{GetOrderByIdInput, GetOrderByIdUseCase},
    interfaces::{
        app_state::AppState,
        order::{errors::OrderInterError, handlers::get_by_id::dto::OrderGetByIdResponse},
    },
};

pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
) -> Result<impl IntoResponse, OrderInterError> {
    let result = GetOrderByIdUseCase::new(app_state.order_repo)
        .execute(GetOrderByIdInput { id })
        .await?;
    Ok((StatusCode::OK, Json(OrderGetByIdResponse::from(result))))
}
