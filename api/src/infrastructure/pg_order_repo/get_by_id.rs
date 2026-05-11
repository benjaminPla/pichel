use crate::{
    domain::order::{ports::repository::OrderRepoError, value_objects::id::OrderId, Order, OrderItem},
    infrastructure::pg_order_repo::row::{OrderItemRow, OrderRow},
};
use sqlx::PgPool;

pub async fn get_by_id(pool: &PgPool, order_id: &OrderId) -> Result<(Order, Vec<OrderItem>), OrderRepoError> {
    let order_row = sqlx::query_as::<_, OrderRow>(
        "SELECT id, customer_phone, customer_email, customer_name, total_price_cents, status, created_at
         FROM orders WHERE id = $1",
    )
    .bind(order_id.value())
    .fetch_one(pool)
    .await?;

    let item_rows = sqlx::query_as::<_, OrderItemRow>(
        "SELECT id, order_id, product_id, product_name, price_cents_at_time, quantity, sale_mode
         FROM order_items WHERE order_id = $1",
    )
    .bind(order_id.value())
    .fetch_all(pool)
    .await?;

    let order      = Order::try_from(order_row)?;
    let order_items = item_rows.into_iter().map(OrderItem::try_from).collect::<Result<Vec<_>, _>>()?;
    Ok((order, order_items))
}
