use axum::{extract::State, response::IntoResponse, Json};
use uuid::Uuid;

struct ProductCreateRequest {
    description: Option<String>,
    image_url: Option<String>,
    low_stock_threshold: u32,
    name: String,
    price_cents: u64,
    stock: u32,
    symbols: Vec<String>,
    unit_of_measure: String,
}

struct ProductCreateResponse {
    description: Option<String>,
    id: Uuid,
    image_url: Option<String>,
    low_stock_threshold: u32,
    name: String,
    price_cents: u64,
    stock: u32,
    symbols: Vec<String>,
    unit_of_measure: String,
}

pub async fn product_create(
    State(app_state): State<AppState>,
    Json(body): Json<ProductCreateRequest>,
) -> Result<impl IntoResponse, ProductInterError> {
    todo!()
}
