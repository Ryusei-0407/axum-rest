run: lint
	cargo run -q

lint: fmt
	cargo clippy

fmt:
	cargo fmt
