UNAME_M := $(shell uname -m)

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
	@if [ "$(UNAME_M)" = "arm64" ]; then \
		image="cosmwasm/optimizer-arm64"; \
	else \
		image="cosmwasm/optimizer"; \
	fi; \
	docker run --rm -v $(CURDIR)/../../:/code:Z --workdir /code/contracts/template \
		--mount type=volume,source=template_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		$$image:0.15.0