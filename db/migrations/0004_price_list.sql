CREATE TABLE IF NOT EXISTS app_settings (
    key        VARCHAR(50) PRIMARY KEY,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by UUID REFERENCES users(id)
);

INSERT INTO app_settings (key) VALUES ('price_list_updated_at') ON CONFLICT DO NOTHING;

CREATE OR REPLACE FUNCTION fn_touch_price_list()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE app_settings SET updated_at = NOW() WHERE key = 'price_list_updated_at';
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_price_list_updated ON products;
CREATE TRIGGER trg_price_list_updated
AFTER INSERT OR UPDATE OR DELETE ON products
FOR EACH STATEMENT
EXECUTE FUNCTION fn_touch_price_list();
