.PHONY: fmt db-init db-stop watch clippy run db-url db-connect redis-init

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings

db-init:
	chmod +x ./scripts/init_db.sh && ./scripts/init_db.sh

docker-stop:
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

db-connect:
	docker exec -it $$(docker ps -q --filter ancestor=postgres) psql -U postgres -d urlshortener

redis-init:
	chmod +x ./scripts/init_redis.sh && ./scripts/init_redis.sh

build:
	cargo build --release

prod-run:
	RUST_LOG=trace ./target/release/urlshortner