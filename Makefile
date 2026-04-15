.PHONY: dev-up dev-down db-migrate db-seed db-exec clean logs

dev-up:
	docker compose up -d db web
	cd api && \
	DATABASE_URL=postgres://pichel:pichel@localhost:5432/pichel \
	JWT_SECRET=dev-secret-change-in-prod \
	cargo watch -x run

dev-down:
	docker compose down

db-migrate:
	@echo "Waiting for DB to be ready..."
	@until docker compose exec -T db psql -U pichel -d pichel -c '\q' 2>/dev/null; do sleep 1; done
	@echo "Running migrations..."
	docker compose exec -T db psql -U pichel -d pichel < db/migrations/0001_init.sql
	@echo "Migrations done."

db-exec:
	docker compose exec db psql -U pichel -d pichel

db-seed:
	@echo "Running seed..."
	docker compose exec -T db psql -U pichel -d pichel < db/seed.sql
	@echo "Done. Login: admin@admin.com / admin"

clean:
	docker compose down
	rm -rf /tmp/pichel-pg
	@echo "Wiped."

logs:
	docker compose logs -f
