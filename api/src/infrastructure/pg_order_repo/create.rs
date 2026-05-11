use crate::{
    domain::order::{ports::repository::OrderRepoError, Order, OrderItem},
    infrastructure::pg_order_repo::row::{OrderItemRow, OrderRow},
};
use sqlx::PgPool;

pub async fn create(pool: &PgPool, order: &Order, items: &[OrderItem]) -> Result<(Order, Vec<OrderItem>), OrderRepoError> {
    let mut tx = pool.begin().await?;

    let order_row = sqlx::query_as::<_, OrderRow>(
        "INSERT INTO orders (id, customer_phone, customer_email, customer_name, total_price_cents, status)
         VALUES ($1,$2,$3,$4,$5,'pending')
         RETURNING id, customer_phone, customer_email, customer_name, total_price_cents, status, created_at",
    )
    .bind(order.get_id().value())
    .bind(order.get_customer_phone())
    .bind(order.get_customer_email())
    .bind(order.get_customer_name())
    .bind(order.get_total_price_cents().value() as i32)
    .fetch_one(&mut *tx)
    .await?;

    let mut item_rows = Vec::with_capacity(items.len());
    for item in items {
        let row = sqlx::query_as::<_, OrderItemRow>(
            "INSERT INTO order_items (id, order_id, product_id, product_name, price_cents_at_time, quantity, sale_mode)
             VALUES ($1,$2,$3,$4,$5,$6,$7)
             RETURNING id, order_id, product_id, product_name, price_cents_at_time, quantity, sale_mode",
        )
        .bind(item.get_id().value())
        .bind(item.get_order_id().value())
        .bind(item.get_product_id().value())
        .bind(item.get_product_name())
        .bind(item.get_price_cents_at_time().value() as i32)
        .bind(item.get_quantity().value() as i32)
        .bind(item.get_sale_mode().as_str())
        .fetch_one(&mut *tx)
        .await?;
        item_rows.push(row);
    }

    tx.commit().await?;

    let order        = Order::try_from(order_row)?;
    let order_items  = item_rows.into_iter().map(OrderItem::try_from).collect::<Result<Vec<_>, _>>()?;
    Ok((order, order_items))
}
