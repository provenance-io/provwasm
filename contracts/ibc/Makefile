.PHONY: all
all: local remote

.PHONY: local
attrs:
	@make -C local

.PHONY: remote
marker:
	@make -C remote

.PHONY: clean
clean:
	@make -C local clean
	@make -C remote clean
