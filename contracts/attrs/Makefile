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
	@cargo test

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

.PHONY: check
check:
	@cargo check

.PHONY: optimize
optimize:
	@docker run --rm -v $(CURDIR):/code:Z -v "$(CURDIR)/../../packages":/packages:Z \
		--mount type=volume,source=attrs_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.12.10

.PHONY: install
install:
	@cp artifacts/attrs.wasm $(PROVWASM_INSTALL_DIR)
