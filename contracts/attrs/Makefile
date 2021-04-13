.PHONY: all
all: fmt build test schema

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

.PHONY: clean
clean:
	@cargo clean

.PHONY: check
check:
	@cargo check

.PHONY: optimize
optimize:
	@docker run --rm -v $(CURDIR):/code -v "$(CURDIR)/../../packages":/packages \
		--mount type=volume,source=attrs_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.11.0

.PHONY: install
install: optimize
	@cp artifacts/attrs.wasm $(PROVWASM_INSTALL_DIR)
