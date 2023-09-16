.PHONY: fmt db-init db-stop watch

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings

db-init:
	chmod +x ./scripts/init_db.sh && ./scripts/init_db.sh

db-stop:
	echo "stopping all running containers"
	docker stop $$(docker ps -q)

watch:
	echo "increasing number of open files to 10000 \n"
	ulimit -n 10000
	export DATABASE_URL="postgres://postgres:password@localhost:54321/urlshortener" 
	RUST_LOG=trace cargo watch -x check -x test -x run

run:
	RUST_LOG=trace cargo run

db-url:
	@echo "$(DATABASE_URL)"
