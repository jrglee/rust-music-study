.DEFAULT_GOAL := prepush

prepush: format test build

format:
	cargo fmt

build:
	cargo build

test:
	cargo test
