use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    application::user::{
        commands::create::{UserCreateCommand, UserCreateHandler},
        queries::get_all::{UserGetAllHandler, UserGetAllQuery},
    },
    interfaces::{
        app_state::AppState,
        pagination::{GetAllQueryParams, MAX_PER_PAGE},
    },
};
use super::{
    dto::{UserCreateRequestBody, UserCreateResponse, UserGetAllItem, UserGetAllResponse},
    errors::UserInterError,
};

// ── Create ───────────────────────────────────────────────────────────────

pub async fn user_create(
    State(app_state): State<AppState>,
    Json(body):       Json<UserCreateRequestBody>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UserCreateHandler::new(app_state.user_hasher, app_state.user_repo)
        .execute(UserCreateCommand {
            email:    body.email,
            password: body.password,
        })
        .await?;
    Ok((StatusCode::CREATED, Json(UserCreateResponse::from(user))))
}

// ── GetAll ───────────────────────────────────────────────────────────────

pub async fn user_get_all(
    State(app_state): State<AppState>,
    Query(query):     Query<GetAllQueryParams>,
) -> Result<impl IntoResponse, UserInterError> {
    let (users, total) = UserGetAllHandler::new(app_state.user_repo)
        .execute(UserGetAllQuery {
            page:     query.page.max(1),
            per_page: query.per_page.clamp(1, MAX_PER_PAGE),
        })
        .await?;
    let users = users.into_iter().map(UserGetAllItem::from).collect::<Vec<_>>();
    Ok((StatusCode::OK, Json(UserGetAllResponse { users, total })))
}
