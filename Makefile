.PHONY: dev-up dev-down db-seed db-exec logs

dev-up:
	docker compose up -d db
	cd web && npm run dev &
	cd api && \
	DATABASE_URL=postgres://pichel:pichel@localhost:5432/pichel \
	PORT=3000 \
	RUST_LOG=info \
	JWT_SECRET=dev-secret-change-in-prod \
	cargo watch -x run

dev-down:
	docker compose down

db-exec:
	docker compose exec db psql -U pichel -d pichel

db-seed:
	@echo "Running seed..."
	docker compose exec -T db psql -U pichel -d pichel < db/seed.sql
	@echo "Done. Login: admin@admin.com / admin12345!"

logs:
	docker compose logs -f
