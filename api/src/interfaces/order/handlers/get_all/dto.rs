use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::order::{Order, OrderItem};

#[derive(Serialize)]
pub struct OrderItemResponse {
    id:                  Uuid,
    price_cents_at_time: u32,
    product_id:          Uuid,
    product_name:        String,
    quantity:            u32,
    sale_mode:           String,
}

impl From<OrderItem> for OrderItemResponse {
    fn from(i: OrderItem) -> Self {
        Self {
            id:                  i.get_id().value(),
            price_cents_at_time: i.get_price_cents_at_time().value(),
            product_id:          i.get_product_id().value(),
            product_name:        i.get_product_name().to_string(),
            quantity:            i.get_quantity().value(),
            sale_mode:           i.get_sale_mode().as_str().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct OrderGetAllItem {
    created_at:        DateTime<Utc>,
    customer_email:    String,
    customer_name:     Option<String>,
    customer_phone:    String,
    id:                Uuid,
    items:             Vec<OrderItemResponse>,
    status:            String,
    total_price_cents: u32,
}

impl From<(Order, Vec<OrderItem>)> for OrderGetAllItem {
    fn from((o, items): (Order, Vec<OrderItem>)) -> Self {
        Self {
            created_at:        o.get_created_at(),
            customer_email:    o.get_customer_email().to_string(),
            customer_name:     o.get_customer_name().map(|s| s.to_string()),
            customer_phone:    o.get_customer_phone().to_string(),
            id:                o.get_id().value(),
            items:             items.into_iter().map(OrderItemResponse::from).collect(),
            status:            o.get_status().as_str().to_string(),
            total_price_cents: o.get_total_price_cents().value(),
        }
    }
}

#[derive(Serialize)]
pub struct OrderGetAllResponse {
    pub orders: Vec<OrderGetAllItem>,
    pub total:  i64,
}

#[derive(Deserialize)]
pub struct GetAllOrdersQueryParams {
    pub page:     i64,
    pub per_page: i64,
}

impl GetAllOrdersQueryParams {
    pub const MAX_PER_PAGE: i64 = 50;
}
