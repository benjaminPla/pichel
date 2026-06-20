use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::order::{
    ports::repository::OrderRepoError,
    value_objects::{
        email::Email,
        id::{OrderId, OrderItemId},
        order_status::OrderStatus,
        quantity::Quantity,
    },
    Order, OrderItem,
};
use crate::domain::product::value_objects::{
    id::ProductId,
    price_cents::PriceCents,
    sale_mode::SaleMode,
};

#[derive(sqlx::FromRow)]
pub struct OrderRow {
    pub created_at:        DateTime<Utc>,
    pub customer_email:    Option<String>,
    pub customer_name:     Option<String>,
    pub customer_phone:    String,
    pub id:                Uuid,
    pub status:            String,
    pub total_price_cents: i32,
}

impl TryFrom<OrderRow> for Order {
    type Error = OrderRepoError;

    fn try_from(r: OrderRow) -> Result<Order, OrderRepoError> {
        let id                = OrderId::reconstitute(r.id);
        let status            = r.status.parse::<OrderStatus>()?;
        let total_price_cents = PriceCents::new(u32::try_from(r.total_price_cents)?)?;
        let customer_email    = r.customer_email
            .map(Email::new)
            .transpose()
            .map_err(|e| OrderRepoError::Mapping(e.to_string()))?;
        Ok(Order::reconstitute(r.created_at, customer_email, r.customer_name, r.customer_phone, id, status, total_price_cents))
    }
}

#[derive(sqlx::FromRow)]
pub struct OrderItemRow {
    pub id:                  Uuid,
    pub order_id:            Uuid,
    pub price_cents_at_time: i32,
    pub product_id:          Uuid,
    pub product_name:        String,
    pub quantity:            i32,
    pub sale_mode:           String,
}

impl TryFrom<OrderItemRow> for OrderItem {
    type Error = OrderRepoError;

    fn try_from(r: OrderItemRow) -> Result<OrderItem, OrderRepoError> {
        let id                  = OrderItemId::reconstitute(r.id);
        let order_id            = OrderId::reconstitute(r.order_id);
        let price_cents_at_time = PriceCents::new(u32::try_from(r.price_cents_at_time)?)?;
        let product_id          = ProductId::reconstitute(r.product_id);
        let quantity            = Quantity::new(u32::try_from(r.quantity)?)?;
        let sale_mode           = r.sale_mode.parse::<SaleMode>()?;
        Ok(OrderItem::reconstitute(id, order_id, price_cents_at_time, product_id, r.product_name, quantity, sale_mode))
    }
}
