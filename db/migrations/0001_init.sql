CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id            UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    email         VARCHAR(40)  NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE products (
    id                  UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    name                VARCHAR(100) NOT NULL,
    description         VARCHAR(255),
    price_cents         INTEGER      NOT NULL CHECK (price_cents > 0),
    unit_of_measure     VARCHAR(20)  NOT NULL,
    sale_mode           VARCHAR(20)  NOT NULL CHECK (sale_mode IN ('bulk', 'package')),
    stock               INTEGER      NOT NULL DEFAULT 0 CHECK (stock >= 0),
    image_url           VARCHAR(255),
    symbols             TEXT[]       NOT NULL DEFAULT ARRAY[]::TEXT[],
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
