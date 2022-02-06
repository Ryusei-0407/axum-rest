run: lint
	cargo run

lint: fmt
	cargo clippy

fmt:
	cargo fmt

setup:
	docker-compose up -d --build &&\
	sqlx db create && sqlx migrate run &&\
	docker build . -t axum-api && docker run --rm -p 3000:3000 -d axum-api

clean:
	sqlx db drop &&\
	docker-compose down
