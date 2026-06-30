use std::sync::Arc;
use uuid::Uuid;
use crate::{
    application::order::errors::OrderAppError,
    domain::{
        order::{
            ports::repository::OrderRepo,
            value_objects::{id::OrderId, order_status::OrderStatus, phone::Phone, quantity::Quantity},
            Order, OrderItem,
        },
        product::{
            ports::repository::ProductRepo,
            value_objects::{id::ProductId, price_cents::PriceCents, sale_mode::SaleMode},
        },
        shared::value_objects::email::Email,
    },
};

pub struct UpdateOrderItemInput {
    pub product_id: Uuid,
    pub quantity:   u32,
}

pub struct UpdateOrderInput {
    pub id:             Uuid,
    pub customer_email: Option<Option<String>>, // None = keep, Some(None) = clear, Some(Some(_)) = set
    pub customer_name:  Option<Option<String>>,
    pub customer_phone: Option<String>,
    pub items:          Option<Vec<UpdateOrderItemInput>>,
    pub status:         Option<String>,
}

pub struct UpdateOrderUseCase {
    order_repo:   Arc<dyn OrderRepo>,
    product_repo: Arc<dyn ProductRepo>,
}

impl UpdateOrderUseCase {
    pub fn new(order_repo: Arc<dyn OrderRepo>, product_repo: Arc<dyn ProductRepo>) -> Self {
        Self { order_repo, product_repo }
    }

    pub async fn execute(&self, input: UpdateOrderInput) -> Result<(Order, Vec<OrderItem>), OrderAppError> {
        let order_id      = OrderId::reconstitute(input.id);
        let (current, current_items) = self.order_repo.get_by_id(&order_id).await?;

        // ── Status ──────────────────────────────────────────────────────────
        let new_status = match input.status {
            Some(s) => {
                let parsed = s.parse::<OrderStatus>()?;
                current.get_status().validate_transition(&parsed)
                    .map_err(|e| OrderAppError::Validation(e.to_string()))?;
                parsed
            }
            None => current.get_status().clone(),
        };

        // ── Customer fields ─────────────────────────────────────────────────
        let customer_phone = match input.customer_phone {
            Some(p) => Phone::new(p)?,
            None    => Phone::new(current.get_customer_phone())?,
        };

        let customer_email = match input.customer_email {
            None             => current.get_customer_email().map(|e| Email::new(e)).transpose()?,
            Some(None)       => None,
            Some(Some(s))    => Some(Email::new(s)?),
        };

        let customer_name = match input.customer_name {
            None          => current.get_customer_name().map(str::to_string),
            Some(None)    => None,
            Some(Some(s)) => Some(s),
        };

        // ── Items + total ────────────────────────────────────────────────────
        let (new_items, total_price_cents) = match input.items {
            None => (None, current.get_total_price_cents()),

            Some(item_inputs) => {
                if item_inputs.is_empty() {
                    return Err(OrderAppError::Validation("order must have at least one item".to_string()));
                }

                struct Snapshot {
                    price_cents:  PriceCents,
                    product_id:   ProductId,
                    product_name: String,
                    quantity:     Quantity,
                    sale_mode:    SaleMode,
                }

                let mut snapshots: Vec<Snapshot> = Vec::with_capacity(item_inputs.len());
                let mut total_cents: u64 = 0;

                for ii in item_inputs {
                    let product  = self.product_repo
                        .get_by_id(&ProductId::reconstitute(ii.product_id))
                        .await
                        .map_err(|_| OrderAppError::Validation(format!("product {} not found", ii.product_id)))?;
                    let quantity = Quantity::new(ii.quantity)?;
                    total_cents += compute_subtotal(product.get_price_cents().value(), quantity.value(), product.get_sale_mode()) as u64;
                    snapshots.push(Snapshot {
                        price_cents:  product.get_price_cents(),
                        product_id:   product.get_id().clone(),
                        product_name: product.get_name().value().to_string(),
                        quantity,
                        sale_mode:    product.get_sale_mode().clone(),
                    });
                }

                let total = PriceCents::new(
                    u32::try_from(total_cents).map_err(|_| OrderAppError::Validation("total price overflow".to_string()))?,
                )?;

                let items: Vec<OrderItem> = snapshots
                    .into_iter()
                    .map(|s| OrderItem::new(order_id.clone(), s.price_cents, s.product_id, s.product_name, s.quantity, s.sale_mode))
                    .collect();

                (Some(items), total)
            }
        };

        let updated = Order::reconstitute(
            current.get_created_at(),
            customer_email,
            customer_name,
            customer_phone,
            order_id,
            new_status,
            total_price_cents,
        );

        let result = self.order_repo.update(&updated, new_items.as_deref()).await?;

        // When items were not replaced, return the original items (infra re-fetches them)
        let _ = current_items;
        Ok(result)
    }
}

fn compute_subtotal(price_cents: u32, quantity: u32, sale_mode: &SaleMode) -> u32 {
    match sale_mode {
        SaleMode::Unit => price_cents * quantity,
        SaleMode::Bulk => ((price_cents as u64 * quantity as u64 + 999) / 1000) as u32,
    }
}
