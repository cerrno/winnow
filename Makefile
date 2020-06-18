test:
	cargo t --verbose

check:
	cargo check
	cargo fmt -- --check
	cargo clippy 

.PHONY: test check


