# Name Module Integration Test

This a CosmWasm smart contract that tests the Rust bindings and mocks for the provenance `name`
module.

This smart contract tests the following functionality

- Messages
  - Bind a name to the contract address (init)
  - Bind a child name under the contract root name (handle)
  - Unbind a child name from the contract root name (handle)
- Queries
  - Resolve the address for any name
  - Lookup all names bound to any address

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

Store the name integration test smart contract WASM in provenance

```bash
provenance tx wasm store artifacts/name.wasm \
    --source "https://github.com/provenance-io/provwasm/tree/main/contracts/name" \
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

Instantiate the contract and bind the name `name-itv2.pb` to it's address

```bash
provenance tx wasm instantiate 1 '{"name": "name-itv2.pb"}' \
    --admin $(provenance keys show -a provenance --keyring-backend test) \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --label name_module_integration_test_v1 \
    --gas auto \
    --fees 3200vspn \
    --broadcast-mode block \
    -y -o json | jq
```

Execute a name resolve query

```bash
provenance query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz '{"resolve":{"name":"name-itv2.pb"}}' | jq
 ```

Execute a name lookup query

```bash
provenance query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"lookup":{"address":"tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"}}' | jq
```

Execute the contract, sending a prefix to bind under the contract root name.

```bash
pio tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"bind_prefix":{"prefix":"nm"}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3500vspn \
    --broadcast-mode block \
    -o json -y | jq
```

To unbind the prefix

```bash
pio tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"unbind_prefix":{"prefix":"nm"}}' \
    --from provenance \
    --keyring-backend test \
    --chain-id pio-dev-chain \
    --gas auto \
    --fees 3500vspn \
    --broadcast-mode block \
    -o json -y | jq
```
