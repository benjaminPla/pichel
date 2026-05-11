use std::collections::HashMap;
use uuid::Uuid;
use crate::{
    domain::order::{ports::repository::OrderRepoError, Order, OrderItem},
    infrastructure::pg_order_repo::row::{OrderItemRow, OrderRow},
};
use sqlx::PgPool;

pub async fn list(pool: &PgPool, page: i64, per_page: i64) -> Result<(Vec<(Order, Vec<OrderItem>)>, i64), OrderRepoError> {
    let offset     = (page - 1) * per_page;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM orders")
        .fetch_one(pool)
        .await
        .map_err(|e| OrderRepoError::Database(e.to_string()))?;

    let order_rows = sqlx::query_as::<_, OrderRow>(
        "SELECT id, customer_phone, customer_email, customer_name, total_price_cents, status, created_at
         FROM orders ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    if order_rows.is_empty() {
        return Ok((vec![], total));
    }

    let order_ids: Vec<Uuid> = order_rows.iter().map(|r| r.id).collect();
    let item_rows = sqlx::query_as::<_, OrderItemRow>(
        "SELECT id, order_id, product_id, product_name, price_cents_at_time, quantity, sale_mode
         FROM order_items WHERE order_id = ANY($1)",
    )
    .bind(&order_ids)
    .fetch_all(pool)
    .await?;

    let mut items_by_order: HashMap<Uuid, Vec<OrderItem>> = HashMap::new();
    for row in item_rows {
        let order_id = row.order_id;
        let item     = OrderItem::try_from(row)?;
        items_by_order.entry(order_id).or_default().push(item);
    }

    let orders = order_rows
        .into_iter()
        .map(|row| {
            let id    = row.id;
            let order = Order::try_from(row)?;
            let items = items_by_order.remove(&id).unwrap_or_default();
            Ok((order, items))
        })
        .collect::<Result<Vec<_>, OrderRepoError>>()?;

    Ok((orders, total))
}
