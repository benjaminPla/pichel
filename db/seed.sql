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
-- image_url points to /images/products/<slug>.webp served by nginx
-- In production the same path is backed by the 40 GB Hetzner volume mount
INSERT INTO products (name, price_cents, unit_amount, unit_type, image_url) VALUES
  ('Avena integral',        420,  '500g',  1, '/images/products/avena_integral.webp'),
  ('Granola sin azúcar',    890,  '400g',  1, '/images/products/granola.webp'),
  ('Arroz integral',        350,  '1kg',   1, '/images/products/arroz_integral.webp'),
  ('Lentejas',              290,  '500g',  1, '/images/products/lentejas.webp'),
  ('Garbanzos',             310,  '500g',  1, '/images/products/garbanzos.webp'),
  ('Quinoa orgánica',       980,  '500g',  1, '/images/products/quinoa.webp'),
  ('Chía',                  750,  '250g',  1, '/images/products/chia.webp'),
  ('Semillas de lino',      620,  '300g',  1, '/images/products/semillas_lino.webp'),
  ('Almendras crudas',     1450,  '200g',  1, '/images/products/almendras.webp'),
  ('Nueces',               1380,  '200g',  1, '/images/products/nueces.webp'),
  ('Aceite de coco',       1200,  '500ml', 1, '/images/products/aceite_coco.webp'),
  ('Aceite de oliva extra', 980,  '500ml', 1, '/images/products/aceite_oliva.webp'),
  ('Miel pura de abeja',    760,  '350g',  1, '/images/products/miel.webp'),
  ('Stevia en polvo',       540,  '100g',  1, '/images/products/stevia.webp'),
  ('Cúrcuma molida',        390,  '100g',  1, '/images/products/curcuma.webp'),
  ('Jengibre en polvo',     360,  '100g',  1, '/images/products/jengibre.webp'),
  ('Proteína de suero',    2100,  '500g',  1, '/images/products/proteina_suero.webp'),
  ('Proteína vegana',      2350,  '500g',  1, '/images/products/proteina_vegana.webp'),
  ('Barritas de cereal',    180,  '1 u.',  1, '/images/products/barritas_cereal.webp'),
  ('Té verde orgánico',     650,  '50 sb', 1, '/images/products/te_verde.webp')
ON CONFLICT DO NOTHING;
