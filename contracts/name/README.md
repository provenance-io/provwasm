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
build/provenanced tx wasm store artifacts/name.wasm \
    --source "https://github.com/provenance-io/provwasm/tree/main/contracts/name" \
    --builder "cosmwasm/rust-optimizer:0.10.7" \
    --instantiate-only-address $(build/provenanced keys show -a validator --keyring-backend test --home build/run/provenanced) \
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto \
    --fees 25000nhash \
    --broadcast-mode block \
    --yes | jq
```

Instantiate the contract and bind the name `name-itv2.pb` to it's address

```bash
build/provenanced tx wasm instantiate 1 '{"name": "name-itv2.pb"}' \
    --admin $(build/provenanced keys show -a validator --keyring-backend test --home build/run/provenanced) \
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --label name_module_integration_test_v1 \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes | jq
```

Execute a name resolve query

```bash
build/provenanced query wasm contract-state smart \
    pb18vd8fpwxzck93qlwghaj6arh4p7c5n894vnu5g '{"resolve":{"name":"name-itv2.pb"}}' -o json | jq
 ```

Execute a name lookup query

```bash
build/provenanced query wasm contract-state smart \
    pb18vd8fpwxzck93qlwghaj6arh4p7c5n894vnu5g \
    '{"lookup":{"address":"pb18vd8fpwxzck93qlwghaj6arh4p7c5n894vnu5g"}}' -o json  | jq
```

Execute the contract, sending a prefix to bind under the contract root name.

```bash
build/provenanced tx wasm execute \
    pb18vd8fpwxzck93qlwghaj6arh4p7c5n894vnu5g \
    '{"bind_prefix":{"prefix":"nm"}}' \
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes | jq
```

To unbind the prefix

```bash
build/provenanced tx wasm execute \
    pb18vd8fpwxzck93qlwghaj6arh4p7c5n894vnu5g \
    '{"unbind_prefix":{"prefix":"nm"}}' \
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes | jq
```
