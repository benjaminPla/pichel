DO $$ BEGIN
    ALTER TABLE orders ALTER COLUMN customer_email DROP NOT NULL;
EXCEPTION WHEN OTHERS THEN NULL;
END $$;

DO $$ BEGIN
    ALTER TABLE orders ADD CONSTRAINT orders_customer_email_format
        CHECK (customer_email IS NULL OR customer_email ~ '^[^@\s]+@[^@\s]+\.[^@\s]+$');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    ALTER TABLE orders ADD CONSTRAINT orders_customer_email_length
        CHECK (customer_email IS NULL OR LENGTH(customer_email) <= 254);
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    ALTER TABLE orders ALTER COLUMN customer_email TYPE VARCHAR(254);
EXCEPTION WHEN OTHERS THEN NULL;
END $$;
