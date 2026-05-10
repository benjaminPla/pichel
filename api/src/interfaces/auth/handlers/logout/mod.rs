use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use crate::interfaces::app_state::AppState;

pub async fn logout(State(app_state): State<AppState>) -> impl IntoResponse {
    let cookie = format!(
        "token=; HttpOnly{}; SameSite=Strict; Path=/; Max-Age=0",
        if app_state.cookie_secure { "; Secure" } else { "" },
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).unwrap_or_else(|_| {
            HeaderValue::from_static("token=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0")
        }),
    );
    (StatusCode::OK, headers)
}
