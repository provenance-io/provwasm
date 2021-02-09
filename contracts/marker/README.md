# Marker Module Integration Test

This a CosmWasm smart contract that tests the Rust bindings and mocks for the provenance `marker`
module.

This contract has the following functionality.

- Messages
  - Bind a name to the contract's address
  - Create a marker
  - Grant access (all permissions) to a marker
  - Finalize a marker
  - Activate a marker
  - Withdraw coins from a marker
- Queries
  - Get marker by address
  - Get marker by denom

## Build

Compile and optimize the smart contract WASM.

```bash
make && make optimize
```

## Example Usage

_NOTE: Address bech32 values and other params may vary._

First, copy the optimized WASM to your provenance blockchain project root.
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
provenanced tx wasm store marker.wasm \
    --source "https://github.com/provenance-io/provwasm/tree/main/contracts/marker" \
    --builder "cosmwasm/rust-optimizer:0.10.7" \
    --instantiate-only-address $(provenanced keys show -a node0 --keyring-backend test --home build/node0 --testnet) \
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

Instantiate the contract and bind the name `marker-itv2.sc.pb` to it's address

```bash
provenanced tx wasm instantiate 1 '{"name":"marker-itv2.sc.pb"}' \
    --admin $(provenanced keys show -a node0 --keyring-backend test --home build/node0 --testnet) \
    --label marker_module_integration_test_v2 \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 3500nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Execute the contract, creating a marker in a 'proposed' state.

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"create_marker":{"coin":{"denom":"nugz","amount":"420"}, "marker_type":"coin"}}' \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 3500nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Query the marker by denom

```bash
provenanced q wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"get_by_denom":{"denom":"nugz"}}' \
    --testnet -o json | jq
 ```

Query the marker by address

```bash
provenanced q wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"get_by_address": { "address": "tp1085qhetuel3vxwk7k345w4g4t5qves9tkfcjht"}}' \
    --testnet -o json | jq
```

Grant access to the marker, so the contract can withdraw funds in a later step

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"grant_access":{"denom":"nugz","address":"tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"}}' \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 3500nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Finalize the marker

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"finalize":{"denom":"nugz"}}' \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 3500nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Activate the marker

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"activate":{"denom":"nugz"}}' \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 3600nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Withdraw coins from the marker to the smart contract instance

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"withdraw":{"coin":{"denom":"nugz","amount":"400"},"recipient":"tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"}}' \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 3500nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```
