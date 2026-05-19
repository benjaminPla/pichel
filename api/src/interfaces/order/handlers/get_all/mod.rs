pub mod dto;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};
use crate::{
    application::order::get_all::{GetAllOrdersInput, GetAllOrdersUseCase},
    interfaces::{
        app_state::AppState,
        order::{errors::OrderInterError, handlers::get_all::dto::{GetAllOrdersQueryParams, OrderGetAllItem, OrderGetAllResponse}},
    },
};

pub async fn get_all(
    State(app_state): State<AppState>,
    Query(query):     Query<GetAllOrdersQueryParams>,
) -> Result<impl IntoResponse, OrderInterError> {
    let (orders, total) = GetAllOrdersUseCase::new(app_state.order_repo)
        .execute(GetAllOrdersInput {
            page:     query.page.max(1),
            per_page: query.per_page.clamp(1, GetAllOrdersQueryParams::MAX_PER_PAGE),
        })
        .await?;
    let orders = orders.into_iter().map(OrderGetAllItem::from).collect::<Vec<_>>();
    Ok((StatusCode::OK, Json(OrderGetAllResponse { orders, total })))
}
