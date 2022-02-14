# Metadata Module Integration Test

This a CosmWasm smart contract that tests the Rust bindings and mocks for querying the provenance
metadata module.

This smart contract tests the following functionality

- Messages
  - Bind a name to the contact address
- Queries
  - Query scope by ID
  - Query scope sessions
  - Query scope records
  - Query scope records by name

## Build

Compile and optimize the smart contract Wasm.

```bash
make && make optimize
```

## Setup

_NOTE: Address bech32 values and other params may vary._

First, copy the optimized Wasm to your Provenance Blockchain project root.
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
    --fees 1000000000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Store the metadata integration test smart contract Wasm in provenance

```bash
provenanced tx wasm store scope.wasm \
    --instantiate-only-address $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 10000000000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Instantiate the contract and bind the metadata `scope-itv2.sc.pb` to it's address

```bash
provenanced tx wasm instantiate 1 '{"name": "scope-itv2.sc.pb"}' \
    --admin $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    --label metadata_module_integration_test_v2 \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 1000000000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

## Example Queries

Execute a scope query

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"get_scope":{"id":"scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d"}}' -t -o json | jq
```

Execute a scope sessions query

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"get_sessions":{"scope_id":"scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d"}}' -t -o json | jq
```

Execute a scope records query

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"get_records":{"scope_id":"scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d"}}' -t -o json | jq
```

Execute a scope records query by name

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"get_records_by_name":{"scope_id":"scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d","name":"loan"}}' -t -o json | jq
```
