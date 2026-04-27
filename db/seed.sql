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
-- symbols: see SYMBOL_DEFS in web/index.html
INSERT INTO products (name, price_cents, cost_price, unit_amount, unit_type, stock, low_stock_threshold, image_url, symbols) VALUES
  ('Avena integral',        420,  260,  '500g',  1, 32, 5, '/images/products/avena_integral.webp',  ARRAY['vegan','sin_tacc','sin_azucar','sin_sal']),
  ('Granola sin azúcar',    890,  550,  '400g',  1, 18, 5, '/images/products/granola.webp',         ARRAY['vegan','sin_azucar']),
  ('Arroz integral',        350,  210,  '1kg',   1,  0, 5, '/images/products/arroz_integral.webp',  ARRAY['vegan','sin_tacc','sin_sal']),
  ('Lentejas',              290,  170,  '500g',  1, 44, 5, '/images/products/lentejas.webp',        ARRAY['vegan','sin_tacc','alto_proteina','sin_sal']),
  ('Garbanzos',             310,  185,  '500g',  1,  3, 5, '/images/products/garbanzos.webp',       ARRAY['vegan','sin_tacc','alto_proteina','sin_sal']),
  ('Quinoa orgánica',       980,  620,  '500g',  1, 11, 5, '/images/products/quinoa.webp',          ARRAY['vegan','sin_tacc','organico','alto_proteina','sin_sal']),
  ('Chía',                  750,  470,  '250g',  1,  0, 5, '/images/products/chia.webp',            ARRAY['vegan','sin_tacc','sin_sal']),
  ('Semillas de lino',      620,  380,  '300g',  1, 27, 5, '/images/products/semillas_lino.webp',   ARRAY['vegan','sin_tacc','sin_sal']),
  ('Almendras crudas',     1450,  920,  '200g',  1,  4, 5, '/images/products/almendras.webp',       ARRAY['vegan','sin_tacc','frutos_secos']),
  ('Nueces',               1380,  870,  '200g',  1, 15, 5, '/images/products/nueces.webp',          ARRAY['vegan','sin_tacc','frutos_secos']),
  ('Aceite de coco',       1200,  760,  '500ml', 2,  8, 5, '/images/products/aceite_coco.webp',     ARRAY['vegan','sin_tacc','sin_lactosa','organico']),
  ('Aceite de oliva extra', 980,  610,  '500ml', 2, 20, 5, '/images/products/aceite_oliva.webp',    ARRAY['vegan','sin_tacc','sin_lactosa']),
  ('Miel pura de abeja',    760,  480,  '350g',  1, 13, 5, '/images/products/miel.webp',            ARRAY['sin_tacc','sin_sal']),
  ('Stevia en polvo',       540,  330,  '100g',  1,  2, 5, '/images/products/stevia.webp',          ARRAY['vegan','sin_tacc','sin_azucar','sin_sal']),
  ('Cúrcuma molida',        390,  240,  '100g',  1, 30, 5, '/images/products/curcuma.webp',         ARRAY['vegan','sin_tacc','organico','sin_sal']),
  ('Jengibre en polvo',     360,  220,  '100g',  1, 22, 5, '/images/products/jengibre.webp',        ARRAY['vegan','sin_tacc','organico','sin_sal']),
  ('Proteína de suero',    2100, 1350,  '500g',  1,  6, 5, '/images/products/proteina_suero.webp',  ARRAY['sin_tacc','alto_proteina']),
  ('Proteína vegana',      2350, 1500,  '500g',  1,  0, 5, '/images/products/proteina_vegana.webp', ARRAY['vegan','sin_tacc','sin_lactosa','alto_proteina']),
  ('Barritas de cereal',    180,  100,  '1 u.',  3, 48, 5, '/images/products/barritas_cereal.webp', ARRAY['vegan','sin_azucar']),
  ('Té verde orgánico',     650,  400,  '50 sb', 3, 16, 5, '/images/products/te_verde.webp',        ARRAY['vegan','sin_tacc','organico','sin_azucar','sin_sal'])
ON CONFLICT DO NOTHING;
