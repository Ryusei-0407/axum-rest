run: lint
	cargo run

lint: fmt
	cargo clippy

fmt:
	cargo fmt
