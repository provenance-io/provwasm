.PHONY: all
all: fmt build test

.PHONY: fmt
fmt:
	@cargo fmt

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@RUST_BACKTRACE=1 cargo unit-test

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: clean
clean:
	@cargo clean
