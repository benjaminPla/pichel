CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id            UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    email         VARCHAR(40)  NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE products (
    id                  UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    name                VARCHAR(255) NOT NULL,
    description         VARCHAR(500),
    price_cents         INTEGER      NOT NULL CHECK (price_cents >= 0),
    cost_price          INTEGER               CHECK (cost_price >= 0),   -- what the store paid; null = unknown
    unit_amount         VARCHAR(40)  NOT NULL,
    unit_type           INTEGER      NOT NULL,                            -- 1=weight 2=volume 3=unit
    stock               INTEGER      NOT NULL DEFAULT 0 CHECK (stock >= 0),
    low_stock_threshold INTEGER      NOT NULL DEFAULT 5 CHECK (low_stock_threshold >= 0),
    image_url           VARCHAR(255),
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
