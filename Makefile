.DEFAULT_GOAL := prepush

prepush: format lint test build

format:
	cargo fmt

lint:
	cargo clippy -- -D warnings

build:
	cargo build

test:
	cargo test
