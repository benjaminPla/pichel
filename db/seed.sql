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

-- unit_type: 1=weight 2=volume 3=unit
-- cost_price: what the store paid (≈60-70% of price_cents, typical markup)
-- stock: current on-hand units
-- low_stock_threshold: show "stock bajo" warning below this qty
INSERT INTO products (name, price_cents, cost_price, unit_amount, unit_type, stock, low_stock_threshold, image_url) VALUES
  ('Avena integral',        420,  260,  '500g',  1, 32, 5, '/images/products/avena_integral.webp'),
  ('Granola sin azúcar',    890,  550,  '400g',  1, 18, 5, '/images/products/granola.webp'),
  ('Arroz integral',        350,  210,  '1kg',   1,  0, 5, '/images/products/arroz_integral.webp'),
  ('Lentejas',              290,  170,  '500g',  1, 44, 5, '/images/products/lentejas.webp'),
  ('Garbanzos',             310,  185,  '500g',  1,  3, 5, '/images/products/garbanzos.webp'),
  ('Quinoa orgánica',       980,  620,  '500g',  1, 11, 5, '/images/products/quinoa.webp'),
  ('Chía',                  750,  470,  '250g',  1,  0, 5, '/images/products/chia.webp'),
  ('Semillas de lino',      620,  380,  '300g',  1, 27, 5, '/images/products/semillas_lino.webp'),
  ('Almendras crudas',     1450,  920,  '200g',  1,  4, 5, '/images/products/almendras.webp'),
  ('Nueces',               1380,  870,  '200g',  1, 15, 5, '/images/products/nueces.webp'),
  ('Aceite de coco',       1200,  760,  '500ml', 2,  8, 5, '/images/products/aceite_coco.webp'),
  ('Aceite de oliva extra', 980,  610,  '500ml', 2, 20, 5, '/images/products/aceite_oliva.webp'),
  ('Miel pura de abeja',    760,  480,  '350g',  1, 13, 5, '/images/products/miel.webp'),
  ('Stevia en polvo',       540,  330,  '100g',  1,  2, 5, '/images/products/stevia.webp'),
  ('Cúrcuma molida',        390,  240,  '100g',  1, 30, 5, '/images/products/curcuma.webp'),
  ('Jengibre en polvo',     360,  220,  '100g',  1, 22, 5, '/images/products/jengibre.webp'),
  ('Proteína de suero',    2100, 1350,  '500g',  1,  6, 5, '/images/products/proteina_suero.webp'),
  ('Proteína vegana',      2350, 1500,  '500g',  1,  0, 5, '/images/products/proteina_vegana.webp'),
  ('Barritas de cereal',    180,  100,  '1 u.',  3, 48, 5, '/images/products/barritas_cereal.webp'),
  ('Té verde orgánico',     650,  400,  '50 sb', 3, 16, 5, '/images/products/te_verde.webp')
ON CONFLICT DO NOTHING;
