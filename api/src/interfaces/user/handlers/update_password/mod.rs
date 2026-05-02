pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use crate::{
    application::user::commands::update_password::{UserUpdatePasswordCommand, UserUpdatePasswordHandler},
    interfaces::{
        app_state::AppState,
        user::{
            errors::UserInterError,
            handlers::update_password::dto::{UserUpdatePasswordRequestBody, UserUpdatePasswordResponse},
        },
    },
};

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
