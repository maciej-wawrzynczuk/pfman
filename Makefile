.PHONY: check clippy test run test-http

check:
	cargo check

clippy:
	cargo clippy -- -W clippy::pedantic

test:
	cargo test

run:
	cargo run
