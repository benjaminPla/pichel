pub mod ports;
pub mod value_objects;

use chrono::{DateTime, Utc};
use value_objects::{
    id::{OrderId, OrderItemId},
    order_status::OrderStatus,
    quantity::Quantity,
};
use crate::domain::product::value_objects::{
    id::ProductId,
    price_cents::PriceCents,
    sale_mode::SaleMode,
};

#[derive(Debug, Clone)]
pub struct OrderItem {
    id:                  OrderItemId,
    order_id:            OrderId,
    price_cents_at_time: PriceCents,
    product_id:          ProductId,
    product_name:        String,
    quantity:            Quantity,
    sale_mode:           SaleMode,
}

impl OrderItem {
    pub fn new(
        order_id:            OrderId,
        price_cents_at_time: PriceCents,
        product_id:          ProductId,
        product_name:        String,
        quantity:            Quantity,
        sale_mode:           SaleMode,
    ) -> Self {
        Self {
            id: OrderItemId::new(),
            order_id,
            price_cents_at_time,
            product_id,
            product_name,
            quantity,
            sale_mode,
        }
    }

    pub fn reconstitute(
        id:                  OrderItemId,
        order_id:            OrderId,
        price_cents_at_time: PriceCents,
        product_id:          ProductId,
        product_name:        String,
        quantity:            Quantity,
        sale_mode:           SaleMode,
    ) -> Self {
        Self { id, order_id, price_cents_at_time, product_id, product_name, quantity, sale_mode }
    }

    pub fn get_id(&self)                  -> &OrderItemId { &self.id }
    pub fn get_order_id(&self)            -> &OrderId     { &self.order_id }
    pub fn get_price_cents_at_time(&self) -> PriceCents   { self.price_cents_at_time }
    pub fn get_product_id(&self)          -> &ProductId   { &self.product_id }
    pub fn get_product_name(&self)        -> &str         { &self.product_name }
    pub fn get_quantity(&self)            -> Quantity      { self.quantity }
    pub fn get_sale_mode(&self)           -> &SaleMode     { &self.sale_mode }
}

#[derive(Debug, Clone)]
pub struct Order {
    created_at:        DateTime<Utc>,
    customer_email:    String,
    customer_name:     Option<String>,
    customer_phone:    String,
    id:                OrderId,
    status:            OrderStatus,
    total_price_cents: PriceCents,
}

impl Order {
    pub fn new(
        customer_email:    String,
        customer_name:     Option<String>,
        customer_phone:    String,
        total_price_cents: PriceCents,
    ) -> Self {
        Self {
            created_at:     Utc::now(),
            customer_email,
            customer_name,
            customer_phone,
            id:             OrderId::new(),
            status:         OrderStatus::Pending,
            total_price_cents,
        }
    }

    pub fn reconstitute(
        created_at:        DateTime<Utc>,
        customer_email:    String,
        customer_name:     Option<String>,
        customer_phone:    String,
        id:                OrderId,
        status:            OrderStatus,
        total_price_cents: PriceCents,
    ) -> Self {
        Self { created_at, customer_email, customer_name, customer_phone, id, status, total_price_cents }
    }

    pub fn get_created_at(&self)        -> DateTime<Utc>  { self.created_at }
    pub fn get_customer_email(&self)    -> &str           { &self.customer_email }
    pub fn get_customer_name(&self)     -> Option<&str>   { self.customer_name.as_deref() }
    pub fn get_customer_phone(&self)    -> &str           { &self.customer_phone }
    pub fn get_id(&self)                -> &OrderId      { &self.id }
    pub fn get_status(&self)            -> &OrderStatus  { &self.status }
    pub fn get_total_price_cents(&self) -> PriceCents    { self.total_price_cents }
}
