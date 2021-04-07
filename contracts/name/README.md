# Name Module Integration Test

This a CosmWasm smart contract that tests the Rust bindings and mocks for the provenance `name`
module.

This smart contract tests the following functionality

- Messages
  - Bind a name to the contract address
  - Bind a child name under the contract root name
  - Unbind a child name from the contract root name
- Queries
  - Resolve the address for any name
  - Lookup all names bound to any address

## Build

Compile and optimize the smart contract WASM.

```bash
make && make optimize
```

## Usage

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

Store the name integration test smart contract WASM in provenance

```bash
provenanced tx wasm store name.wasm \
    --source "https://github.com/provenance-io/provwasm/tree/main/contracts/name" \
    --builder "cosmwasm/rust-optimizer:0.10.9" \
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

Instantiate the contract and bind the name `name-itv2.sc.pb` to it's address

```bash
provenanced tx wasm instantiate 1 '{"name": "name-itv2.sc.pb"}' \
    --admin $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    --label name_module_integration_test_v2 \
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

Execute a name resolve query

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"resolve":{"name":"name-itv2.sc.pb"}}' -t -o json | jq
 ```

Execute a name lookup query

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"lookup":{"address":"tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"}}' -t -o json  | jq
```

Execute the contract, sending a prefix to bind under the contract root name.

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"bind_prefix":{"prefix":"nm"}}' \
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

To unbind the prefix

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"unbind_prefix":{"prefix":"nm"}}' \
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
