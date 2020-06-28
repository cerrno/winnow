test:
	cargo t --verbose

lint:
	cargo clean
	cargo clippy -- -D warnings
	cargo fmt -- --check

.PHONY: test lint


