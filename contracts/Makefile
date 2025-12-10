.PHONY: all
all: attrs ibc marker name scope trigger tutorial

.PHONY: attrs
attrs:
	@make -C attrs

.PHONY: ibc
ibc:
	@make -C ibc

.PHONY: marker
marker:
	@make -C marker

.PHONY: name
name:
	@make -C name

.PHONY: scope
scope:
	@make -C scope

.PHONY: trigger
trigger:
	@make -C trigger

.PHONY: tutorial
tutorial:
	@make -C tutorial

.PHONY: build
build:
	@make -C attrs build
	@make -C ibc build
	@make -C marker build
	@make -C name build
	@make -C scope build
	@make -C trigger build
	@make -C tutorial build

.PHONY: optimize
optimize:
	@make -C attrs optimize
	@make -C ibc optimize
	@make -C marker optimize
	@make -C name optimize
	@make -C scope optimize
	@make -C trigger optimize
	@make -C tutorial optimize

.PHONY: clean
clean:
	@make -C attrs clean
	@make -C ibc clean
	@make -C marker clean
	@make -C name clean
	@make -C scope clean
	@make -C trigger clean
	@make -C tutorial clean
