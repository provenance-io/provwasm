.PHONY: all
all: provwasm-std contracts

.PHONY: provwasm-std
provwasm-std:
	@make -C packages/provwasm-std

.PHONY: contracts
contracts:
	@make -C contracts

.PHONY: clean
clean:
	@make -C packages/provwasm-std clean
	@make -C contracts clean

.PHONY: tutorial
tutorial:
	@make -C contracts/tutorial pre-optimize

.PHONY: optimize-tutorial
optimize-tutorial:
	@make -C contracts/tutorial optimize

.PHONY: attrs
attrs:
	@make -C contracts/attrs

.PHONY: marker
marker:
	@make -C contracts/marker

.PHONY: msgfees
msgfees:
	@make -C contracts/msgfees

.PHONY: name
name:
	@make -C contracts/name

.PHONY: scope
scope:
	@make -C contracts/scope

.PHONY: optimize-contracts
test-attrs: attrs
	if [[ $(arch) == "arm64" ]]; then
	  image="cosmwasm/workspace-optimizer-arm64"
	else
	  image="cosmwasm/workspace-optimizer"
	fi

	docker run --rm -v "$(pwd)":/code:Z \
	  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
	  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	  ${image}:0.12.13
