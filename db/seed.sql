-- ──────────────────────────────────────────────────────────────
-- Seed data for local development
-- Run with: make seed   (after: make dev)
--
-- Credentials:  admin@admin.com / admin12345!
-- ──────────────────────────────────────────────────────────────

INSERT INTO users (email, password_hash) VALUES (
  'admin@admin.com',
  '$argon2id$v=19$m=12288,t=3,p=1$b3iAk6wn8u8ga1qdiRNuAg$LkAdkLI3JmHiw64v4r/KKFk+Yl8CnRlRNcfw8SrbVtM'
) ON CONFLICT (email) DO UPDATE SET password_hash = EXCLUDED.password_hash;

-- unit_of_measure: g | kg | ml | l | unit
-- symbols: vegan | vegetarian | gluten_free | lactose_free | organic | sugar_free | contains_nuts | high_protein | no_added_salt
INSERT INTO products (name, price_cents, unit_of_measure, sale_mode, image_url, symbols) VALUES
  ('Avena integral',        420,  'g',    'bulk',    '/images/products/avena_integral.webp',  ARRAY['vegan','gluten_free','sugar_free','no_added_salt']),
  ('Granola sin azúcar',    890,  'g',    'bulk',    '/images/products/granola.webp',         ARRAY['vegan','sugar_free']),
  ('Arroz integral',        350,  'kg',   'bulk',    '/images/products/arroz_integral.webp',  ARRAY['vegan','gluten_free','no_added_salt']),
  ('Lentejas',              290,  'g',    'bulk',    '/images/products/lentejas.webp',        ARRAY['vegan','gluten_free','high_protein','no_added_salt']),
  ('Garbanzos',             310,  'g',    'bulk',    '/images/products/garbanzos.webp',       ARRAY['vegan','gluten_free','high_protein','no_added_salt']),
  ('Quinoa orgánica',       980,  'g',    'bulk',    '/images/products/quinoa.webp',          ARRAY['vegan','gluten_free','organic','high_protein','no_added_salt']),
  ('Chía',                  750,  'g',    'bulk',    '/images/products/chia.webp',            ARRAY['vegan','gluten_free','no_added_salt']),
  ('Semillas de lino',      620,  'g',    'bulk',    '/images/products/semillas_lino.webp',   ARRAY['vegan','gluten_free','no_added_salt']),
  ('Almendras crudas',     1450,  'g',    'bulk',    '/images/products/almendras.webp',       ARRAY['vegan','gluten_free','contains_nuts']),
  ('Nueces',               1380,  'g',    'bulk',    '/images/products/nueces.webp',          ARRAY['vegan','gluten_free','contains_nuts']),
  ('Aceite de coco',       1200,  'ml',   'bulk',    '/images/products/aceite_coco.webp',     ARRAY['vegan','gluten_free','lactose_free','organic']),
  ('Aceite de oliva extra', 980,  'ml',   'bulk',    '/images/products/aceite_oliva.webp',    ARRAY['vegan','gluten_free','lactose_free']),
  ('Miel pura de abeja',    760,  'g',    'bulk',    '/images/products/miel.webp',            ARRAY['gluten_free','no_added_salt']),
  ('Stevia en polvo',       540,  'g',    'bulk',    '/images/products/stevia.webp',          ARRAY['vegan','gluten_free','sugar_free','no_added_salt']),
  ('Cúrcuma molida',        390,  'g',    'bulk',    '/images/products/curcuma.webp',         ARRAY['vegan','gluten_free','organic','no_added_salt']),
  ('Jengibre en polvo',     360,  'g',    'bulk',    '/images/products/jengibre.webp',        ARRAY['vegan','gluten_free','organic','no_added_salt']),
  ('Proteína de suero',    2100,  'g',    'bulk',    '/images/products/proteina_suero.webp',  ARRAY['gluten_free','high_protein']),
  ('Proteína vegana',      2350,  'g',    'bulk',    '/images/products/proteina_vegana.webp', ARRAY['vegan','gluten_free','lactose_free','high_protein']),
  ('Barritas de cereal',    180,  'unit', 'package', '/images/products/barritas_cereal.webp', ARRAY['vegan','sugar_free']),
  ('Té verde orgánico',     650,  'unit', 'package', '/images/products/te_verde.webp',        ARRAY['vegan','gluten_free','organic','sugar_free','no_added_salt'])
ON CONFLICT DO NOTHING;
