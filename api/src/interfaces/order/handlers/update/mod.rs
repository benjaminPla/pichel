pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use uuid::Uuid;
use crate::{
    application::order::update::{UpdateOrderInput, UpdateOrderItemInput, UpdateOrderUseCase},
    domain::auth::Claims,
    interfaces::{
        app_state::AppState,
        order::{errors::OrderInterError, handlers::update::dto::{OrderUpdateRequestBody, OrderUpdateResponse}},
    },
};

pub async fn update(
    State(app_state):  State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id):          Path<Uuid>,
    Json(body):        Json<OrderUpdateRequestBody>,
) -> Result<impl IntoResponse, OrderInterError> {
    let _ = claims;
    let result = UpdateOrderUseCase::new(app_state.order_repo, app_state.product_repo)
        .execute(UpdateOrderInput {
            id,
            customer_email: body.customer_email,
            customer_name:  body.customer_name,
            customer_phone: body.customer_phone,
            status:         body.status,
            items:          body.items.map(|items| items.into_iter()
                .map(|i| UpdateOrderItemInput { product_id: i.product_id, quantity: i.quantity })
                .collect()),
        })
        .await?;
    Ok((StatusCode::OK, Json(OrderUpdateResponse::from(result))))
}
