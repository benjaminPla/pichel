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

-- unit_of_measure: kg | unit
-- symbols: vegan | vegetarian | gluten_free | lactose_free | organic | sugar_free | contains_nuts | high_protein | no_added_salt
INSERT INTO products (active, name, plu, price_cents, unit_of_measure, sale_mode, image_url, symbols) VALUES
  (true, 'Avena integral',        1,  420,  'kg',   'bulk',    '/images/products/avena_integral.webp',  ARRAY['vegan','gluten_free','sugar_free','no_added_salt']),
  (true, 'Granola sin azúcar',    2,  890,  'kg',   'bulk',    '/images/products/granola.webp',         ARRAY['vegan','sugar_free']),
  (true, 'Arroz integral',        3,  350,  'kg',   'bulk',    '/images/products/arroz_integral.webp',  ARRAY['vegan','gluten_free','no_added_salt']),
  (true, 'Lentejas',              4,  290,  'kg',   'bulk',    '/images/products/lentejas.webp',        ARRAY['vegan','gluten_free','high_protein','no_added_salt']),
  (true, 'Garbanzos',             5,  310,  'kg',   'bulk',    '/images/products/garbanzos.webp',       ARRAY['vegan','gluten_free','high_protein','no_added_salt']),
  (true, 'Quinoa orgánica',       6,  980,  'kg',   'bulk',    '/images/products/quinoa.webp',          ARRAY['vegan','gluten_free','organic','high_protein','no_added_salt']),
  (true, 'Chía',                  7,  750,  'kg',   'bulk',    '/images/products/chia.webp',            ARRAY['vegan','gluten_free','no_added_salt']),
  (true, 'Semillas de lino',      8,  620,  'kg',   'bulk',    '/images/products/semillas_lino.webp',   ARRAY['vegan','gluten_free','no_added_salt']),
  (true, 'Almendras crudas',      9, 1450,  'kg',   'bulk',    '/images/products/almendras.webp',       ARRAY['vegan','gluten_free','contains_nuts']),
  (true, 'Nueces',               10, 1380,  'kg',   'bulk',    '/images/products/nueces.webp',          ARRAY['vegan','gluten_free','contains_nuts']),
  (true, 'Aceite de coco',       11, 1200,  'kg',   'bulk',    '/images/products/aceite_coco.webp',     ARRAY['vegan','gluten_free','lactose_free','organic']),
  (true, 'Aceite de oliva extra',12,  980,  'kg',   'bulk',    '/images/products/aceite_oliva.webp',    ARRAY['vegan','gluten_free','lactose_free']),
  (true, 'Miel pura de abeja',   13,  760,  'kg',   'bulk',    '/images/products/miel.webp',            ARRAY['gluten_free','no_added_salt']),
  (true, 'Stevia en polvo',      14,  540,  'kg',   'bulk',    '/images/products/stevia.webp',          ARRAY['vegan','gluten_free','sugar_free','no_added_salt']),
  (true, 'Cúrcuma molida',       15,  390,  'kg',   'bulk',    '/images/products/curcuma.webp',         ARRAY['vegan','gluten_free','organic','no_added_salt']),
  (true, 'Jengibre en polvo',    16,  360,  'kg',   'bulk',    '/images/products/jengibre.webp',        ARRAY['vegan','gluten_free','organic','no_added_salt']),
  (true, 'Proteína de suero',    17, 2100,  'kg',   'bulk',    '/images/products/proteina_suero.webp',  ARRAY['gluten_free','high_protein']),
  (true, 'Proteína vegana',      18, 2350,  'kg',   'bulk',    '/images/products/proteina_vegana.webp', ARRAY['vegan','gluten_free','lactose_free','high_protein']),
  (true, 'Barritas de cereal',   19,  180,  'unit', 'unit', '/images/products/barritas_cereal.webp', ARRAY['vegan','sugar_free']),
  (true, 'Té verde orgánico',    20,  650,  'unit', 'unit', '/images/products/te_verde.webp',        ARRAY['vegan','gluten_free','organic','sugar_free','no_added_salt'])
ON CONFLICT DO NOTHING;
