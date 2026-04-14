-- ──────────────────────────────────────────────────────────────
-- Seed data for local development
-- Run with: make seed   (after: make dev)
--
-- Credentials:  admin@admin.com / admin
-- ──────────────────────────────────────────────────────────────

INSERT INTO users (email, password_hash) VALUES (
  'admin@admin.com',
  '$argon2id$v=19$m=65536,t=3,p=4$gG+j/TbNvG5Yns0UbtpPYQ$WCPpCLbe3AjG+9ZZFzV1S5lptovXbyiJ3xZzrIO8RQ0'
) ON CONFLICT (email) DO NOTHING;

-- unit_type: 1 = weight — adjust once the enum is defined in code
INSERT INTO products (name, price_cents, unit_amount, unit_type) VALUES
  ('Wagyu Ribeye',       4500, '200g',     1),
  ('Organic Salmon',     2250, '1 fillet', 1),
  ('Free-Range Chicken', 1490, '1kg',      1),
  ('Black Angus Burger', 1800, '500g',     1),
  ('Smoked Pork Ribs',   2100, '500g',     1),
  ('Wild Caught Shrimp', 1950, '300g',     1)
ON CONFLICT DO NOTHING;
