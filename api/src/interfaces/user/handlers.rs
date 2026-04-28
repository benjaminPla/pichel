use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    application::user::{
        commands::user_create::{UserCreateCommand, UserCreateHandler},
        queries::user_get_all::{UserGetAllHandler, UserGetAllQuery},
    },
    interfaces::app_state::AppState,
};

use super::{
    dto::{
        UserCreateRequestBody, UserCreateResponse, UserGetAllItem, UserGetAllQueryParams,
        UserGetAllResponse, MAX_PER_PAGE,
    },
    errors::UserInterError,
};

pub async fn user_create(
    State(app_state): State<AppState>,
    Json(body): Json<UserCreateRequestBody>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UserCreateHandler::new(app_state.user_repo).execute(UserCreateCommand {
        email: body.email,
        password: body.password,
    }).await?;
    Ok((StatusCode::CREATED, Json(UserCreateResponse::from(user))))
}
