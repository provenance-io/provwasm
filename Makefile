.PHONY: all
all: bindings mocks contracts

.PHONY: bindings
bindings:
	@make -C packages/bindings

.PHONY: mocks
mocks:
	@make -C packages/mocks

.PHONY: contracts
contracts:
	@make -C contracts

.PHONY: clean
clean:
	@make -C packages/bindings clean
	@make -C packages/mocks clean
	@make -C contracts clean

.PHONY: tutorial
tutorial:
	@make -C contracts/tutorial

.PHONY: build-release-zip
build-release-zip:
	cd contracts/tutorial/artifact && \
	  zip -u provwasm_tutorial.wasm && \
	cd ../../..
