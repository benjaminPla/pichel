pub mod dto;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::{
    application::order::create::{CreateOrderInput, CreateOrderUseCase, OrderItemInput},
    interfaces::{
        app_state::AppState,
        order::{errors::OrderInterError, handlers::create::dto::{OrderCreateRequestBody, OrderCreateResponse}},
    },
};

pub async fn create(
    State(app_state): State<AppState>,
    Json(body):       Json<OrderCreateRequestBody>,
) -> Result<impl IntoResponse, OrderInterError> {
    let result = CreateOrderUseCase::new(app_state.order_repo, app_state.product_repo)
        .execute(CreateOrderInput {
            customer_email: body.customer_email,
            customer_name:  body.customer_name,
            customer_phone: body.customer_phone,
            items: body.items.into_iter().map(|i| OrderItemInput { product_id: i.product_id, quantity: i.quantity }).collect(),
        })
        .await?;
    Ok((StatusCode::CREATED, Json(OrderCreateResponse::from(result))))
}
