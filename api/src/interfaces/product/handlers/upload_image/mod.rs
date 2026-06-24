use axum::{extract::{Multipart, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use image::imageops::FilterType;
use serde_json::json;
use crate::{
    domain::auth::Claims,
    interfaces::{app_state::AppState, product::errors::ProductInterError},
};

const MAX_INPUT_BYTES: usize = 8 * 1024 * 1024; // 8 MB
const OUTPUT_PX: u32 = 400;
const WEBP_QUALITY: u8 = 70;

pub async fn upload_image(
    State(app_state):  State<AppState>,
    Extension(_claims): Extension<Claims>,
    mut multipart:     Multipart,
) -> Result<impl IntoResponse, ProductInterError> {
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut mime: Option<String> = None;

    while let Some(field) = multipart.next_field().await
        .map_err(|e| ProductInterError(StatusCode::BAD_REQUEST, e.to_string()))?
    {
        if field.name() == Some("image") {
            mime = field.content_type().map(|s| s.to_string());
            let bytes = field.bytes().await
                .map_err(|e| ProductInterError(StatusCode::BAD_REQUEST, e.to_string()))?;
            if bytes.len() > MAX_INPUT_BYTES {
                return Err(ProductInterError(StatusCode::BAD_REQUEST, "file exceeds 8 MB limit".into()));
            }
            file_bytes = Some(bytes.to_vec());
        }
    }

    let bytes = file_bytes
        .ok_or_else(|| ProductInterError(StatusCode::BAD_REQUEST, "missing 'image' field".into()))?;

    match mime.as_deref() {
        Some("image/jpeg" | "image/png" | "image/webp") => {}
        _ => return Err(ProductInterError(StatusCode::BAD_REQUEST, "unsupported format — use JPEG, PNG or WebP".into())),
    }

    let img = image::load_from_memory(&bytes)
        .map_err(|_| ProductInterError(StatusCode::BAD_REQUEST, "invalid or corrupt image".into()))?;

    let img = img.resize_to_fill(OUTPUT_PX, OUTPUT_PX, FilterType::Lanczos3);
    let webp_vec = {
        let rgb = img.to_rgb8();
        let encoder = webp::Encoder::from_rgb(rgb.as_raw(), rgb.width(), rgb.height());
        encoder.encode(WEBP_QUALITY as f32).to_vec()
    };

    let filename = format!("{}.webp", uuid::Uuid::new_v4());
    let path = std::path::Path::new(&app_state.uploads_path).join(&filename);
    tokio::fs::write(&path, &webp_vec).await
        .map_err(|e| ProductInterError(StatusCode::INTERNAL_SERVER_ERROR, format!("write error: {e}")))?;

    Ok((StatusCode::CREATED, Json(json!({ "url": format!("/uploads/{filename}") }))))
}
