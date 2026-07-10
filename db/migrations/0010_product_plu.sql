ALTER TABLE products ADD COLUMN plu SMALLINT;

UPDATE products AS p
SET plu = sub.rn
FROM (
    SELECT id, ROW_NUMBER() OVER (ORDER BY created_at) AS rn
    FROM products
) AS sub
WHERE p.id = sub.id;

ALTER TABLE products ALTER COLUMN plu SET NOT NULL;

DO $$ BEGIN
    ALTER TABLE products ADD CONSTRAINT products_plu_range CHECK (plu BETWEEN 1 AND 800);
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    ALTER TABLE products ADD CONSTRAINT products_plu_key UNIQUE (plu);
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;
