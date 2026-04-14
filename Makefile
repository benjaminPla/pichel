.PHONY: dev-up dev-down db-seed db-exec clean logs

dev-up:
	docker compose up -d db web
	cd api && \
	DATABASE_URL=postgres://gordo:gordo@localhost:5432/gordo \
	JWT_SECRET=dev-secret-change-in-prod \
	cargo watch -x run

dev-down:
	docker compose down

db-exec:
	docker compose exec db psql -U gordo -d gordo

db-seed:
	@echo "Waiting for migrations..."
	@until docker compose exec -T db psql -U gordo -d gordo -c '\dt' 2>/dev/null | grep -q users; do \
		sleep 1; \
	done
	@echo "Running seed..."
	docker compose exec -T db psql -U gordo -d gordo < db/seed.sql
	@echo "Done. Login: admin@admin.com / admin"

clean:
	docker compose down
	rm -rf /tmp/gordo-pg
	@echo "Wiped."

logs:
	docker compose logs -f
