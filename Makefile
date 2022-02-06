run: lint
	cargo run

lint: fmt
	cargo clippy

fmt:
	cargo fmt

setup:
	docker-compose up -d --build &&\
	sqlx db create &&\
	sqlx migrate run &&\
	cargo run

clean:
	cargo clean &&\
	sqlx migrate revert &&\
	sqlx db drop &&\
	docker-compose down &&\
