# Metadata Module Integration Test

## Account Attributes

This a CosmWasm smart contract that tests the Rust bindings and mocks for managing account
attributes in the provenance `attribute` module.

This smart contract tests the following functionality

- Messages
  - Bind a name to the contract address
  - Bind label attribute name to contract address (must be done after init, but before add/delete)
  - Add label attribute
  - Delete label attributes
- Queries
  - Get label attribute name
  - Get label attributes

## Build

Compile and optimize the smart contract WASM.

```bash
make && make optimize
```

## Example Usage

_NOTE: Address bech32 values and other params may vary._

First, copy the optimized WASM to your Provenance Blockchain project root.
Then, install the `provenanced` command and genesis a localnet.

```bash
make clean
make install
make localnet-start
```

Create a root name binding for smart contracts (required once per localnet genesis).

```bash
provenanced tx name bind \
    "sc" \
    $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    "pb" \
    --restrict=false \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Store the `attrs` integration test WASM, requiring that only the `node0` account can
instantiate contracts.

```bash
provenanced tx wasm store attrs.wasm \
    --source "https://github.com/provenance-io/provwasm/tree/main/contracts/attrs" \
    --builder "cosmwasm/rust-optimizer:0.11.0" \
    --instantiate-only-address $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 25000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Instantiate the contract, binding the name `attrs-itv2.sc.pb` to the contract address.

```bash
provenanced tx wasm instantiate 1 '{"name": "attrs-itv2.sc.pb"}' \
    --admin $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    --label attribute_module_integration_test_v2 \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Bind the label attribute name under the contract root name. This establishes ownership so label
attributes can be added and deleted in later operations.

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"bind_label_name":{}}' \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Query for the label name just bound.

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz '{"get_label_name":{}}' -t -o json | jq
```

Add labels to an account.

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"add_label":{"text":"hello"}}' \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"add_label":{"text":"wasm"}}' \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Query for the labels set on an account.

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz '{"get_labels":{}}' -t -o json | jq
```

Delete the labels from an account.

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"delete_labels":{}}' \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
``
