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
attrs:
	@make -C contracts/attrs

.PHONY: marker
marker:
	@make -C contracts/marker

.PHONY: name
name:
	@make -C contracts/name

.PHONY: scope
scope:
	@make -C contracts/scope

.PHONY: build-release-zip
build-release-zip: tutorial
	zip provwasm_tutorial.zip ./contracts/tutorial/artifacts/provwasm_tutorial.wasm

PROVENANCE_TEST_VERSION = "v1.8.0"

.PHONY: test-tutorial
test-tutorial: tutorial optimize-tutorial
	docker rm -f test_container || true
	docker pull provenanceio/provenance-testing-action
	docker create --name test_container provenanceio/provenance-testing-action --entrypoint	"/scripts/tutorial_test.sh" "$(PROVENANCE_TEST_VERSION)"
	docker cp ./scripts test_container:/scripts
	docker cp ./contracts test_container:/go/contracts
	docker start test_container

.PHONY: test-attrs
test-attrs: attrs
	docker rm -f test_container || true
	docker pull provenanceio/provenance-testing-action
	docker create --name test_container provenanceio/provenance-testing-action --entrypoint	"/scripts/attrs_test.sh" "$(PROVENANCE_TEST_VERSION)"
	docker cp ./scripts test_container:/scripts
	docker cp ./contracts test_container:/go/contracts
	docker start test_container

.PHONY: test-marker
test-marker: marker
	docker rm -f test_container || true
	docker pull provenanceio/provenance-testing-action
	docker create --name test_container provenanceio/provenance-testing-action --entrypoint	"/scripts/marker_test.sh" "$(PROVENANCE_TEST_VERSION)"
	docker cp ./scripts test_container:/scripts
	docker cp ./contracts test_container:/go/contracts
	docker start test_container

.PHONY: test-name
test-name: name
	docker rm -f test_container || true
	docker pull provenanceio/provenance-testing-action
	docker create --name test_container provenanceio/provenance-testing-action --entrypoint	"/scripts/name_test.sh" "$(PROVENANCE_TEST_VERSION)"
	docker cp ./scripts test_container:/scripts
	docker cp ./contracts test_container:/go/contracts
	docker start test_container

.PHONY: test-scope
test-scope: scope
	docker rm -f test_container || true
	docker pull provenanceio/provenance-testing-action
	docker create --name test_container provenanceio/provenance-testing-action --entrypoint	"/scripts/scope_test.sh" "$(PROVENANCE_TEST_VERSION)"
	docker cp ./scripts test_container:/scripts
	docker cp ./contracts test_container:/go/contracts
	docker start test_container