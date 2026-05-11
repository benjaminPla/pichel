pub mod dto;

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, Json};
use crate::{
    application::order::list::{ListOrdersInput, ListOrdersUseCase},
    interfaces::{
        app_state::AppState,
        order::{errors::OrderInterError, handlers::list::dto::{ListOrdersQueryParams, OrderListItem, OrderListResponse}},
    },
};

pub async fn list(
    State(app_state): State<AppState>,
    Query(query):     Query<ListOrdersQueryParams>,
) -> Result<impl IntoResponse, OrderInterError> {
    let (orders, total) = ListOrdersUseCase::new(app_state.order_repo)
        .execute(ListOrdersInput {
            page:     query.page.max(1),
            per_page: query.per_page.clamp(1, ListOrdersQueryParams::MAX_PER_PAGE),
        })
        .await?;
    let orders = orders.into_iter().map(OrderListItem::from).collect::<Vec<_>>();
    Ok((StatusCode::OK, Json(OrderListResponse { orders, total })))
}
