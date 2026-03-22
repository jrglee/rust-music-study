.DEFAULT_GOAL := prepush

prepush: build test

build:
	cargo build

test:
	cargo test
