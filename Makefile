.PHONY: dev-up dev-down db-migrate db-seed db-exec logs

dev-up:
	docker compose up -d db web
	cd api && \
	DATABASE_URL=postgres://pichel:pichel@localhost:5432/pichel \
	PORT=3000 \
	RUST_LOG=info \
	JWT_SECRET=dev-secret-change-in-prod \
	cargo watch -x run

dev-down:
	docker compose down

db-migrate:
	@echo "Waiting for DB to be ready..."
	@until docker compose exec -T db psql -U pichel -d pichel -c '\q' 2>/dev/null; do sleep 1; done
	@echo "Running migrations..."
	docker compose exec -T db psql -U pichel -d pichel < db/migrations/0001_users.sql
	docker compose exec -T db psql -U pichel -d pichel < db/migrations/0002_products.sql
	docker compose exec -T db psql -U pichel -d pichel < db/migrations/0003_orders.sql
	docker compose exec -T db psql -U pichel -d pichel < db/migrations/0004_price_list.sql
	@echo "Migrations done."

db-exec:
	docker compose exec db psql -U pichel -d pichel

db-seed:
	@echo "Running seed..."
	docker compose exec -T db psql -U pichel -d pichel < db/seed.sql
	@echo "Done. Login: admin@admin.com / admin12345!"

logs:
	docker compose logs -f
