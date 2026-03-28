.PHONY: check clippy test run test-http

check:
	cargo check

clippy:
	cargo clippy -- -W clippy::pedantic

test:
	cargo test

run:
	cargo run

test-http: build
	@echo "Starting server..."
	@RUST_LOG=error ./target/debug/pfman &
	@echo "Waiting for server..."
	@until curl -sf http://127.0.0.1:8080/transactions > /dev/null 2>&1; do sleep 0.1; done
	@echo "Running hurl tests..."
	hurl --test --jobs 1 tests/post_transactions.hurl tests/get_transactions.hurl; \
	STATUS=$$?; \
	kill $$(lsof -ti:8080) 2>/dev/null || true; \
	exit $$STATUS

build:
	cargo build
