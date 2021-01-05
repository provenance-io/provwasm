# Metadata Module Integration Test

## Account Attributes

This a CosmWasm smart contract that tests the Rust bindings and mocks for managing account
attributes in the provenance `metadata` module.

This contract binds a contract root name, then child name to establish that the contract owns the
name used for `Label` attributes. After this setup, labels can be added, deleted, or queried.

This smart contract tests the following functionality

- Messages
  - Bind contract root name to contract address
  - Bind label attribute name to contract address (must be done after init, but before add/delete)
  - Add label attribute
  - Delete label attributes
- Queries
  - Get label attribute name
  - Get label attributes

## Build

Set the install location for the compiled wasm, for example

```bash
export PROVWASM_INSTALL_DIR="$PIO_HOME/artifacts"
```

Compile and install

```bash
make && make install
```

## Example Usage

TODO: Change the commands below to work against a provenance localnet.

_NOTE: Address bech32 values and other params may vary._

Store the `attrs` integration test WASM, requiring that only the `provenance` account can
instantiate contracts.

```bash
provenance tx wasm store artifacts/attrs.wasm \
    --source "https://github.com/FigureTechnologies/provwasm/tree/master/contracts/attrs" \
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

Instantiate the contract, binding the name `attrs-itv2.pb` to the contract address.

```bash
provenance tx wasm instantiate 1 '{"name": "attrs-itv2.pb"}' \
    --admin $(provenance keys show -a provenance --keyring-backend test) \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --label account_module_integration_test_v1 \
    --gas auto \
    --fees 3200vspn \
    --broadcast-mode block \
    -y -o json | jq
```

Bind the label attribute name under the contract root name. This establishes ownership so label
attributes can be added and deleted in later operations.

```bash
pio tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"bind_label_name":{}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3000vspn \
    --broadcast-mode block \
    -o json -y | jq
```

Query for the label name just bound.

```bash
provenance query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz '{"get_label_name":{}}' | jq
```

Add labels to an account.

```bash
pio tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"add_label":{"text":"hello"}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3000vspn \
    --broadcast-mode block \
    -o json -y | jq
```

```bash
pio tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"add_label":{"text":"wasm"}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3000vspn \
    --broadcast-mode block \
    -o json -y | jq
```

Query for the labels set on an account.

```bash
provenance query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"get_labels":{}}' | jq
```

Delete the labels from an account.

```bash
pio tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"delete_labels":{}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3000vspn \
    --broadcast-mode block \
    -o json -y | jq
```
