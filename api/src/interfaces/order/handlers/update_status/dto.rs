use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::order::Order;

#[derive(Deserialize)]
pub struct UpdateStatusRequestBody {
    pub status: String,
}

#[derive(Serialize)]
pub struct OrderUpdateStatusResponse {
    created_at:        DateTime<Utc>,
    customer_email:    String,
    customer_name:     Option<String>,
    customer_phone:    String,
    id:                Uuid,
    status:            String,
    total_price_cents: u32,
}

impl From<Order> for OrderUpdateStatusResponse {
    fn from(o: Order) -> Self {
        Self {
            created_at:        o.get_created_at(),
            customer_email:    o.get_customer_email().to_string(),
            customer_name:     o.get_customer_name().map(|s| s.to_string()),
            customer_phone:    o.get_customer_phone().to_string(),
            id:                o.get_id().value(),
            status:            o.get_status().as_str().to_string(),
            total_price_cents: o.get_total_price_cents().value(),
        }
    }
}
