use crate::{
    domain::order::{
        ports::repository::OrderRepoError,
        value_objects::{id::OrderId, order_status::OrderStatus},
        Order,
    },
    infrastructure::pg_order_repo::row::OrderRow,
};
use sqlx::PgPool;

pub async fn update_status(pool: &PgPool, order_id: &OrderId, status: &OrderStatus) -> Result<Order, OrderRepoError> {
    let row = sqlx::query_as::<_, OrderRow>(
        "UPDATE orders SET status = $2
         WHERE id = $1
         RETURNING id, customer_phone, customer_email, customer_name, total_price_cents, status, created_at",
    )
    .bind(order_id.value())
    .bind(status.as_str())
    .fetch_one(pool)
    .await?;
    Ok(Order::try_from(row)?)
}
