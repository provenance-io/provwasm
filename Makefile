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
	@make -C contracts/tutorial pre-optimize

.PHONY: optimize-tutorial
optimize-tutorial:
	@make -C contracts/tutorial optimize

.PHONY: build-release-zip
build-release-zip: tutorial
	cd ./contracts/tutorial/artifacts && \
	  zip -u provwasm_tutorial.zip provwasm_tutorial.wasm && \
	cd ../../..


.PHONY: build-release-checksum
build-release-checksum:
	cd ./contracts/tutorial/artifacts && \
	  shasum -a 256 *.zip  > sha256sum.txt && \
	cd ../../..

.PHONY: test
test: tutorial
	docker build -t tests .
	docker run tests