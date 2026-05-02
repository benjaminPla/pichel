pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use crate::{
    application::user::queries::get_by_id::{UserGetByIdHandler, UserGetByIdQuery},
    interfaces::{
        app_state::AppState,
        user::{
            errors::UserInterError,
            handlers::get_by_id::dto::UserGetByIdResponse,
        },
    },
};

pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UserGetByIdHandler::new(app_state.user_repo)
        .execute(UserGetByIdQuery { id })
        .await?;
    Ok((StatusCode::OK, Json(UserGetByIdResponse::from(user))))
}
