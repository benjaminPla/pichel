pub mod dto;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{
    application::user::update::{UpdateUserInput, UpdateUserUseCase},
    interfaces::{
        app_state::AppState,
        user::{
            errors::UserInterError,
            handlers::update::dto::{UserUpdateRequestBody, UserUpdateResponse},
        },
    },
};
use uuid::Uuid;

pub async fn update(
    State(app_state): State<AppState>,
    Path(id):         Path<Uuid>,
    Json(body):       Json<UserUpdateRequestBody>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UpdateUserUseCase::new(app_state.user_repo)
        .execute(UpdateUserInput { email: body.email, id })
        .await?;
    Ok((StatusCode::OK, Json(UserUpdateResponse::from(user))))
}
