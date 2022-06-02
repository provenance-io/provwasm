# MsgFees Module Integration Test

This a CosmWasm smart contract that tests the Rust bindings and mocks for the provenance `msgfees`
module.

This smart contract tests the following functionality

- Messages
  - Create an assess custom fee message

## Build

Compile and optimize the smart contract Wasm.

```bash
make && make optimize
```

## Usage

_NOTE: Address bech32 values and other params may vary._

First, copy the optimized Wasm to your Provenance Blockchain project root.
Then, install the `provenanced` command and genesis a localnet.

```bash
make clean
make install
make run
```

Create the accounts used to test
```bash
provenanced keys add sender --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
provenanced keys add feebucket --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
provenanced keys add receiver --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
```

Create a root name binding for smart contracts (required once per localnet genesis).

```bash
provenanced tx name bind \
    "sc" \
    $(provenanced keys show -a node0 --home build/run/provenanced --keyring-backend test --testnet) \
    "pb" \
    --restrict=false \
    --from node0 \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Store the msgfees integration test smart contract Wasm in provenance

```bash
provenanced tx wasm store msgfees.wasm \
    --instantiate-only-address $(provenanced keys show -a node0 --home build/run/provenanced --keyring-backend test --testnet) \
    --from node0 \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto \
    --fees 25000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Instantiate the contract with an assessed fee amount of `10000nhash` split between the `feebucket` account and the `fees` module

```bash
provenanced tx wasm instantiate 1 '{"fee_amount":{"amount":"10000","denom":"nhash"},"fee_recipient":"$(provenanced keys show -a receiver --home build/run/provenanced --keyring-backend test --testnet)"}' \
    --admin $(provenanced keys show -a node0 --home build/run/provenanced --keyring-backend test --testnet) \
    --label msgfees_module_integration_test_v2 \
    --from node0 \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

Execute the contract, sending funds to another account via the contract which assesses fees.

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"send_funds":{"funds":{"amount":"999999","denom":"nhash"},"to_address":"$(provenanced keys show -a receiver --keyring-backend test --home build/run/provenanced -t)"}}' \
    --amount 999999nhash \
    --from sender \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto \
    --fees 105000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```
