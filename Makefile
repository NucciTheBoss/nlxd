.PHONY: test lint fmt check-fmt build release

export RUSTFLAGS = --deny warnings

# the test target should be kept in sync with the CI,
# so running this target should be equivalent to running in CI.
test: lint check-fmt
	cargo test --all-features

lint:
	cargo clippy --all-features

check-fmt:
	cargo fmt --check

fmt:
	cargo fmt

build:
	cargo build

release:
	cargo build --release
