.PHONY: all
all: fmt build test lint schema optimize

.PHONY: pre-optimize
pre-optimize: fmt build test lint schema

UNAME_M := $(shell uname -m)

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
	@if [ "$(UNAME_M)" = "arm64" ]; then \
		image="cosmwasm/optimizer-arm64"; \
	else \
		image="cosmwasm/optimizer"; \
	fi; \
	docker run --rm -v $(CURDIR)/../../:/code:Z --workdir /code/contracts/tutorial \
		--mount type=volume,source=tutorial_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		$$image:0.16.0
