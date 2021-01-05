# Metadata Module Integration Test

## Scopes

This a CosmWasm smart contract that tests the Rust bindings and mocks for the provenance `metadata`
module.

This smart contract tests the following functionality

- Messages
  - Bind a name to the contact address (init)
- Queries
  - Query scope by ID (query)

## Build

Set the install location for the compiled wasm, for example

```bash
export PROVWASM_INSTALL_DIR="$PIO_HOME/artifacts"
```

Compile and install

```bash
make && make install
```

## Usage

TODO: Change the commands below to work against a provenance localnet.

_NOTE: Address bech32 values and other params may vary._

Store the metadata integration test smart contract WASM in provenance

```bash
provenance tx wasm store artifacts/scope.wasm \
    --source "https://github.com/FigureTechnologies/provwasm/tree/master/contracts/scope" \
    --builder "cosmwasm/rust-optimizer:0.10.7" \
    --instantiate-only-address $(provenance keys show -a provenance --keyring-backend test) \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 25000vspn \
    --broadcast-mode block \
    -y -o json | jq
```

Instantiate the contract and bind the metadata `scope-itv2.pb` to it's address

```bash
provenance tx wasm instantiate 1 '{"name": "scope-itv2.pb"}' \
    --admin $(provenance keys show -a provenance --keyring-backend test) \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --label metadata_module_integration_test_v1 \
    --gas auto \
    --fees 3200vspn \
    --broadcast-mode block \
    -y -o json | jq
```

Execute a scope query

```bash
provenance query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"get_scope":{"id":"fbd81e76-fb4b-44f4-98dd-96f78b654f47"}}' | jq
```
