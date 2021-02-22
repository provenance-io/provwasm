.PHONY: all
all: fmt build test lint schema

.PHONY: fmt
fmt:
	@cargo fmt --all -- --check

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@RUST_BACKTRACE=1 cargo unit-test

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: schema
schema:
	@cargo schema

.PHONY: clean
clean:
	@cargo clean
