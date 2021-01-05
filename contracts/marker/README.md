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

Set the install location for the optimized wasm, for example

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

Store the name integration test smart contract WASM in provenance

```bash
provenance tx wasm store artifacts/marker.wasm \
    --source "https://github.com/FigureTechnologies/provwasm/tree/master/contracts/marker" \
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

Instantiate the contract and bind the name `marker-itv2.pb` to it's address

```bash
provenance tx wasm instantiate 1 '{"name":"marker-itv2.pb"}' \
    --admin $(provenance keys show -a provenance --keyring-backend test) \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --label marker_module_integration_test_v1 \
    --gas auto \
    --fees 3500vspn \
    --broadcast-mode block \
    -y -o json | jq
```

Execute the contract, creating a marker in a 'proposed' state.

```bash
provenance tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"create_marker":{"coin":{"denom":"nugz","amount":"420"}}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3500vspn \
    --broadcast-mode block \
    -o json -y | jq
```

Query the marker by denom

```bash
provenance q wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"get_by_denom":{"denom":"nugz"}}' | jq
 ```

Query the marker by address

```bash
provenance q wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"get_by_address": { "address": "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u"}}' | jq
```

Grant access to the marker, so the contract can withdraw funds in a later step

```bash
provenance tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"grant_access":{"denom":"nugz","address":"tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3500vspn \
    --broadcast-mode block \
    -o json -y | jq
```

Finalize the marker

```bash
provenance tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"finalize":{"denom":"nugz"}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3500vspn \
    --broadcast-mode block \
    -o json -y | jq
```

Activate the marker

```bash
provenance tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"activate":{"denom":"nugz"}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3600vspn \
    --broadcast-mode block \
    -o json -y | jq
```

Withdraw coins from the marker

```bash
provenance tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"withdraw":{"coin":{"denom":"nugz","amount":"400"},"recipient":"tp1ugdl868dpz9lt02u5pdv2lr6ql0qjj62fdh6e8"}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3500vspn \
    --broadcast-mode block \
    -o json -y | jq
```
