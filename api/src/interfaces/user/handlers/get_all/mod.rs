pub mod dto;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    application::user::queries::get_all::{UserGetAllHandler, UserGetAllQuery},
    interfaces::{
        app_state::AppState,
        user::{
            errors::UserInterError,
            handlers::get_all::dto::{GetAllQueryParams, UserGetAllItem, UserGetAllResponse},
        },
    },
};

pub async fn get_all(
    State(app_state): State<AppState>,
    Query(query):     Query<GetAllQueryParams>,
) -> Result<impl IntoResponse, UserInterError> {
    let (users, total) = UserGetAllHandler::new(app_state.user_repo)
        .execute(UserGetAllQuery {
            page:     query.page.max(1),
            per_page: query.per_page.clamp(1, GetAllQueryParams::MAX_PER_PAGE),
        })
        .await?;
    let users = users.into_iter().map(UserGetAllItem::from).collect::<Vec<_>>();
    Ok((StatusCode::OK, Json(UserGetAllResponse { total, users })))
}
