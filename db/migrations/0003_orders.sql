CREATE TYPE order_status AS ENUM ('pending', 'closed', 'cancelled');

CREATE TABLE orders (
    id                UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_phone    VARCHAR(30)  NOT NULL,
    customer_email    VARCHAR(100) NOT NULL,
    customer_name     VARCHAR(100),
    total_price_cents INTEGER      NOT NULL CHECK (total_price_cents > 0),
    status            order_status NOT NULL DEFAULT 'pending',
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE order_items (
    id                  UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id            UUID         NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id          UUID         NOT NULL REFERENCES products(id),
    product_name        VARCHAR(100) NOT NULL,
    price_cents_at_time INTEGER      NOT NULL CHECK (price_cents_at_time > 0),
    quantity            INTEGER      NOT NULL CHECK (quantity > 0),
    sale_mode           VARCHAR(20)  NOT NULL
);

CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_orders_status        ON orders(status);
CREATE INDEX idx_orders_created_at    ON orders(created_at DESC);
