use crate::{
    domain::order::{ports::repository::OrderRepoError, Order, OrderItem},
    infrastructure::pg_order_repo::row::{OrderItemRow, OrderRow},
};
use sqlx::PgPool;

pub async fn update(pool: &PgPool, order: &Order, new_items: Option<&[OrderItem]>) -> Result<(Order, Vec<OrderItem>), OrderRepoError> {
    let mut tx = pool.begin().await?;

    let order_row = sqlx::query_as::<_, OrderRow>(
        "UPDATE orders SET
           customer_email    = $2,
           customer_name     = $3,
           customer_phone    = $4,
           status            = $5::order_status,
           total_price_cents = $6
         WHERE id = $1
         RETURNING id, customer_phone, customer_email, customer_name, total_price_cents, status::text AS status, created_at",
    )
    .bind(order.get_id().value())
    .bind(order.get_customer_email())
    .bind(order.get_customer_name())
    .bind(order.get_customer_phone())
    .bind(order.get_status().as_str())
    .bind(order.get_total_price_cents().value() as i32)
    .fetch_one(&mut *tx)
    .await?;

    let item_rows = if let Some(items) = new_items {
        sqlx::query("DELETE FROM order_items WHERE order_id = $1")
            .bind(order.get_id().value())
            .execute(&mut *tx)
            .await
            .map_err(|e| OrderRepoError::Database(e.to_string()))?;

        let mut rows = Vec::with_capacity(items.len());
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
            rows.push(row);
        }
        rows
    } else {
        sqlx::query_as::<_, OrderItemRow>(
            "SELECT id, order_id, product_id, product_name, price_cents_at_time, quantity, sale_mode
             FROM order_items WHERE order_id = $1",
        )
        .bind(order.get_id().value())
        .fetch_all(&mut *tx)
        .await?
    };

    tx.commit().await?;

    let updated_order = Order::try_from(order_row)?;
    let order_items   = item_rows.into_iter().map(OrderItem::try_from).collect::<Result<Vec<_>, _>>()?;
    Ok((updated_order, order_items))
}
