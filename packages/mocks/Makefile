.PHONY: all
all: fmt build test lint

.PHONY: fmt
fmt:
	@cargo fmt --all -- --check

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@cargo test

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: clean
clean:
	@cargo clean
