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
  ('Avena integral',        420,  '500g',  1),
  ('Granola sin azúcar',    890,  '400g',  1),
  ('Arroz integral',        350,  '1kg',   1),
  ('Lentejas',              290,  '500g',  1),
  ('Garbanzos',             310,  '500g',  1),
  ('Quinoa orgánica',       980,  '500g',  1),
  ('Chía',                  750,  '250g',  1),
  ('Semillas de lino',      620,  '300g',  1),
  ('Almendras crudas',     1450,  '200g',  1),
  ('Nueces',               1380,  '200g',  1),
  ('Aceite de coco',       1200,  '500ml', 1),
  ('Aceite de oliva extra', 980,  '500ml', 1),
  ('Miel pura de abeja',    760,  '350g',  1),
  ('Stevia en polvo',       540,  '100g',  1),
  ('Cúrcuma molida',        390,  '100g',  1),
  ('Jengibre en polvo',     360,  '100g',  1),
  ('Proteína de suero',    2100,  '500g',  1),
  ('Proteína vegana',      2350,  '500g',  1),
  ('Barritas de cereal',    180,  '1 u.',  1),
  ('Té verde orgánico',     650,  '50 sb', 1)
ON CONFLICT DO NOTHING;
