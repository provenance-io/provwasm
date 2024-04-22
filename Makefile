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

.PHONY: ibc
ibc:
	@make -C contracts/ibc

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

.PHONY: trigger
trigger:
	@make -C contracts/trigger

.PHONY: integration
integration:
	@make -C integration_tests
