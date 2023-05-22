.PHONY: all
all: fmt build test lint schema optimize

.PHONY: pre-optimize
pre-optimize: fmt build test lint schema

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
	@cargo clippy

.PHONY: schema
schema:
	@cargo schema

.PHONY: clean
clean:
	@cargo clean
	@cargo clean --target-dir artifacts

.PHONY: optimize
optimize:
	@docker run --rm -v $(CURDIR):/code:Z -v "$(CURDIR)/../../packages":/packages:Z \
		--mount type=volume,source=tutorial_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.12.10
