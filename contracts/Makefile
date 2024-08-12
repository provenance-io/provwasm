.PHONY: all
all: attrs ibc marker msgfees name scope trigger tutorial

.PHONY: attrs
attrs:
	@make -C attrs

.PHONY: ibc
ibc:
	@make -C ibc

.PHONY: marker
marker:
	@make -C marker

.PHONY: msgfees
msgfees:
	@make -C msgfees

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

.PHONY: optimize
optimize:
    @make -C attrs optimize
    @make -C ibc optimize
    @make -C marker optimize
    @make -C msgfees optimize
    @make -C name optimize
    @make -C scope optimize
    @make -C trigger optimize
    @make -C tutorial optimize

.PHONY: clean
clean:
	@make -C attrs clean
	@make -C ibc clean
	@make -C marker clean
	@make -C msgfees clean
	@make -C name clean
	@make -C scope clean
	@make -C trigger clean
	@make -C tutorial clean
