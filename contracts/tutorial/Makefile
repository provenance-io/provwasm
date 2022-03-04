.PHONY: all
all: fmt build test lint schema optimize

.PHONY: fmt
fmt:
	@cargo fmt --all -- --check

.PHONY: build
build:
	@cargo wasm

.PHONY: test
test:
	@RUST_BACKTRACE=1 cargo unit-test

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: schema
schema:
	@cargo schema

.PHONY: optimize
optimize:
	@docker run --rm -v $(CURDIR):/code \
		--mount type=volume,source=tutorial_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.11.3
