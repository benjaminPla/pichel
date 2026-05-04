pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    application::user::commands::update_password::{
        UserUpdatePasswordCommand, UserUpdatePasswordHandler,
    },
    interfaces::{
        app_state::AppState,
        auth::AuthUser,
        user::{
            errors::UserInterError,
            handlers::update_password::dto::{
                UserUpdatePasswordRequestBody, UserUpdatePasswordResponse,
            },
        },
    },
};
use uuid::Uuid;

pub async fn update_password(
    _user: AuthUser,
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
    Json(body):       Json<UserUpdatePasswordRequestBody>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UserUpdatePasswordHandler::new(app_state.hasher_service, app_state.user_repo)
        .execute(UserUpdatePasswordCommand { id, password: body.password })
        .await?;
    Ok((StatusCode::OK, Json(UserUpdatePasswordResponse::from(user))))
}
