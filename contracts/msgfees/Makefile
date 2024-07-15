.PHONY: all
all: fmt build test lint schema optimize

UNAME_M := $(shell uname -m)

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
	@if [ "$(UNAME_M)" = "arm64" ]; then \
		image="cosmwasm/optimizer-arm64"; \
	else \
		image="cosmwasm/optimizer"; \
	fi; \
	docker run --rm -v $(CURDIR)/../../:/code:Z --workdir /code/contracts/msgfees \
		--mount type=volume,source=msgfees_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		$$image:0.16.0

.PHONY: install
install:
	@cp artifacts/msgfees.wasm $(PROVWASM_INSTALL_DIR)
