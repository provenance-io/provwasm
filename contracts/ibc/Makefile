.PHONY: all
all: local remote

.PHONY: local
local:
	@make -C local

.PHONY: remote
remote:
	@make -C remote

.PHONY: build
build:
	@make -C local build
	@make -C remote build

.PHONY: optimize
optimize:
	@make -C local optimize
	@make -C remote optimize

.PHONY: clean
clean:
	@make -C local clean
	@make -C remote clean
