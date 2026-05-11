pub mod dto;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;
use crate::{
    application::order::update_status::{UpdateOrderStatusInput, UpdateOrderStatusUseCase},
    interfaces::{
        app_state::AppState,
        order::{errors::OrderInterError, handlers::update_status::dto::{OrderUpdateStatusResponse, UpdateStatusRequestBody}},
    },
};

pub async fn update_status(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
    Json(body):       Json<UpdateStatusRequestBody>,
) -> Result<impl IntoResponse, OrderInterError> {
    let order = UpdateOrderStatusUseCase::new(app_state.order_repo)
        .execute(UpdateOrderStatusInput { id, status: body.status })
        .await?;
    Ok((StatusCode::OK, Json(OrderUpdateStatusResponse::from(order))))
}
