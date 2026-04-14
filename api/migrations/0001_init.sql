CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
    email         VARCHAR(40)  NOT NULL UNIQUE,
    id            UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    password_hash VARCHAR(255) NOT NULL,
);

CREATE TABLE products (
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
    description VARCHAR(500),
    id          UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    image_url   VARCHAR(255), 
    name        VARCHAR(255) NOT NULL,
    price_cents INTEGER      NOT NULL,
    unit_amount VARCHAR(40)  NOT NULL,
    unit_type   INTEGER      NOT NULL,
);
