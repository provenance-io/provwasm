.PHONY: all
all: bindings mocks

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
