pub mod dto;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::{
    application::user::commands::create::{UserCreateCommand, UserCreateHandler},
    interfaces::{
        app_state::AppState,
        auth::AuthUser,
        user::{
            errors::UserInterError,
            handlers::create::dto::{UserCreateRequestBody, UserCreateResponse},
        },
    },
};

pub async fn create(
    _user: AuthUser,
    State(app_state): State<AppState>,
    Json(body):       Json<UserCreateRequestBody>,
) -> Result<impl IntoResponse, UserInterError> {
    let user = UserCreateHandler::new(app_state.hasher_service, app_state.user_repo)
        .execute(UserCreateCommand { email: body.email, password: body.password })
        .await?;
    Ok((StatusCode::CREATED, Json(UserCreateResponse::from(user))))
}
