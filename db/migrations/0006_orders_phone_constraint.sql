DO $$ BEGIN
    ALTER TABLE orders ADD CONSTRAINT orders_customer_phone_format
        CHECK (customer_phone ~ '^\+?[0-9]{7,15}$') NOT VALID;
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;
