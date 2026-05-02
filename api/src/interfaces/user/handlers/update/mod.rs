pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use crate::{
    application::user::commands::update::{UserUpdateCommand, UserUpdateHandler},
    interfaces::{
        app_state::AppState,
        user::{
            errors::UserInterError,
            handlers::update::dto::{UserUpdateRequestBody, UserUpdateResponse},
        },
    },
};

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
