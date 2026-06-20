use std::sync::Arc;
use uuid::Uuid;
use crate::{
    application::order::errors::OrderAppError,
    domain::{
        order::{
            ports::repository::OrderRepo,
            value_objects::quantity::Quantity,
            Order, OrderItem,
        },
        product::{
            ports::repository::ProductRepo,
            value_objects::{id::ProductId, price_cents::PriceCents, sale_mode::SaleMode},
        },
        shared::value_objects::email::Email,
    },
};

pub struct OrderItemInput {
    pub product_id: Uuid,
    pub quantity:   u32,
}

pub struct CreateOrderInput {
    pub customer_email: Option<String>,
    pub customer_name:  Option<String>,
    pub customer_phone: String,
    pub items:          Vec<OrderItemInput>,
}

pub struct CreateOrderUseCase {
    order_repo:   Arc<dyn OrderRepo>,
    product_repo: Arc<dyn ProductRepo>,
}

impl CreateOrderUseCase {
    pub fn new(order_repo: Arc<dyn OrderRepo>, product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { order_repo, product_repo }
    }

    pub async fn execute(&self, input: CreateOrderInput) -> Result<(Order, Vec<OrderItem>), OrderAppError> {
        if input.items.is_empty() {
            return Err(OrderAppError::Validation("order must have at least one item".to_string()));
        }

        struct ItemSnapshot {
            price_cents: PriceCents,
            product_id:  ProductId,
            product_name: String,
            quantity:    Quantity,
            sale_mode:   SaleMode,
        }

        let mut snapshots: Vec<ItemSnapshot> = Vec::with_capacity(input.items.len());
        let mut total_cents: u64 = 0;

        for item_input in input.items {
            let product  = self.product_repo
                .get_by_id(&ProductId::reconstitute(item_input.product_id))
                .await
                .map_err(|_| OrderAppError::Validation(format!("product {} not found", item_input.product_id)))?;
            let quantity = Quantity::new(item_input.quantity)?;
            total_cents += compute_subtotal(product.get_price_cents().value(), quantity.value(), product.get_sale_mode()) as u64;
            snapshots.push(ItemSnapshot {
                price_cents:  product.get_price_cents(),
                product_id:   product.get_id().clone(),
                product_name: product.get_name().value().to_string(),
                quantity,
                sale_mode:    product.get_sale_mode().clone(),
            });
        }

        let total_price_cents = PriceCents::new(
            u32::try_from(total_cents).map_err(|_| OrderAppError::Validation("total price overflow".to_string()))?,
        )?;

        let customer_email = input.customer_email.map(Email::new).transpose()?;
        let order      = Order::new(customer_email, input.customer_name, input.customer_phone, total_price_cents);
        let order_id   = order.get_id().clone();
        let order_items: Vec<OrderItem> = snapshots
            .into_iter()
            .map(|s| OrderItem::new(order_id.clone(), s.price_cents, s.product_id, s.product_name, s.quantity, s.sale_mode))
            .collect();

        let (order, items) = self.order_repo.create(&order, &order_items).await?;
        Ok((order, items))
    }
}

fn compute_subtotal(price_cents: u32, quantity: u32, sale_mode: &SaleMode) -> u32 {
    match sale_mode {
        SaleMode::Unit => price_cents * quantity,
        SaleMode::Bulk => ((price_cents as u64 * quantity as u64 + 999) / 1000) as u32,
    }
}
