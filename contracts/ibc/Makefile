.PHONY: all
all: local remote

.PHONY: local
local:
	@make -C local

.PHONY: remote
remote:
	@make -C remote

.PHONY: clean
clean:
	@make -C local clean
	@make -C remote clean
