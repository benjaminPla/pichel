use serde::Deserialize;

pub const MAX_PER_PAGE: i64 = 50;

#[derive(Deserialize)]
pub struct GetAllQueryParams {
    pub page:     i64,
    pub per_page: i64,
}
