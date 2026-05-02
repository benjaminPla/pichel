use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use super::dto::{
    UserCreateRequestBody, UserCreateResponse,
    UserGetAllItem, UserGetAllResponse,
    UserGetByIdResponse,
    UserUpdatePasswordRequestBody, UserUpdatePasswordResponse,
    UserUpdateRequestBody, UserUpdateResponse,
};
use super::errors::UserInterError;
use crate::application::user::{
    commands::{
        create::{UserCreateCommand, UserCreateHandler},
        update::{UserUpdateCommand, UserUpdateHandler},
        update_password::{UserUpdatePasswordCommand, UserUpdatePasswordHandler},
    },
    queries::{
        get_all::{UserGetAllHandler, UserGetAllQuery},
        get_by_id::{UserGetByIdHandler, UserGetByIdQuery},
    },
};
use crate::interfaces::{
    app_state::AppState,
    pagination::{GetAllQueryParams, MAX_PER_PAGE},
};

// ── Create ───────────────────────────────────────────────────────────────

pub async fn create(
    State(app_state): State<AppState>,
    Json(body):       Json<UserCreateRequestBody>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UserCreateHandler::new(app_state.user_hasher, app_state.user_repo)
        .execute(UserCreateCommand { email: body.email, password: body.password })
        .await?;
    Ok((StatusCode::CREATED, Json(UserCreateResponse::from(user))))
}

// ── Update ───────────────────────────────────────────────────────────────

pub async fn update(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
    Json(body):       Json<UserUpdateRequestBody>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UserUpdateHandler::new(app_state.user_repo)
        .execute(UserUpdateCommand { email: body.email, id })
        .await?;
    Ok((StatusCode::OK, Json(UserUpdateResponse::from(user))))
}

// ── UpdatePassword ────────────────────────────────────────────────────────

pub async fn update_password(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
    Json(body):       Json<UserUpdatePasswordRequestBody>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UserUpdatePasswordHandler::new(app_state.user_hasher, app_state.user_repo)
        .execute(UserUpdatePasswordCommand { id, password: body.password })
        .await?;
    Ok((StatusCode::OK, Json(UserUpdatePasswordResponse::from(user))))
}

// ── GetAll ───────────────────────────────────────────────────────────────

pub async fn get_all(
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
    Ok((StatusCode::OK, Json(UserGetAllResponse { total, users })))
}

// ── GetById ──────────────────────────────────────────────────────────────

pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UserGetByIdHandler::new(app_state.user_repo)
        .execute(UserGetByIdQuery { id })
        .await?;
    Ok((StatusCode::OK, Json(UserGetByIdResponse::from(user))))
}
