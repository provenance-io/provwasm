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

.PHONY: attrs
	@make -C contracts/attrs

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

.PHONY: test-tutorial
test-tutorial: tutorial
	docker build -t tests . --build-arg test_script="./scripts/tutorial_setup.sh" --build-arg contract_location="./contracts/tutorial/artifacts/provwasm_tutorial.wasm" --build-arg contract_destination="provwasm_tutorial.wasm"
	docker run tests "./scripts/tutorial_setup.sh"

.PHONY: test-attrs
test-attrs: attrs
	docker build -t tests . --build-arg test_script="./scripts/attrs_setup.sh" --build-arg contract_location="./contracts/attrs/artifacts/attrs.wasm" --build-arg contract_destination="attrs.wasm"
	docker run tests "./scripts/attrs_setup.sh"