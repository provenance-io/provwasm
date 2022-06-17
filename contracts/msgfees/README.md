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
make clean install run
```

Create the accounts used to test
```bash
provenanced keys add sender --home build/run/provenanced --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
provenanced keys add receiver --home build/run/provenanced --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
provenanced keys add feebucket --home build/run/provenanced --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
```

Fund the sender account
```bash
provenanced tx bank send \
    $(provenanced keys show -a validator --home build/run/provenanced --keyring-backend test --testnet) \
    $(provenanced keys show -a sender --home build/run/provenanced --keyring-backend test --testnet) \
    200000000000nhash \
    --from validator \
    --keyring-backend=test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas=auto \
    --gas-prices="1905nhash" \
    --gas-adjustment=1.5 \
    --broadcast-mode=block \
    --yes \
    --testnet
```

Create a root name binding for smart contracts (required once per localnet genesis).

```bash
provenanced tx name bind \
    "sc" \
    $(provenanced keys show -a validator --home build/run/provenanced --keyring-backend test --testnet) \
    "pb" \
    --restrict=false \
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --broadcast-mode block \
    --fees 10381000000nhash \
    --yes \
    --testnet
```

Store the msgfees integration test smart contract Wasm in provenance

```bash
provenanced tx wasm store msgfees.wasm \
    --instantiate-only-address $(provenanced keys show -a validator --home build/run/provenanced --keyring-backend test --testnet) \
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --fees 3883064370nhash \
    --gas auto --gas-adjustment 1.2 \
    --broadcast-mode block \
    --yes \
    --testnet
```

Instantiate the contract with an assessed fee amount of `10000nhash` split between the `feebucket` account and the `fees` module

```bash
provenanced tx wasm instantiate 1 "{\"fee_amount\":{\"amount\":\"10000\",\"denom\":\"nhash\"},\"fee_recipient\":\"$(provenanced keys show -a feebucket --home build/run/provenanced --keyring-backend test --testnet)\"}" \
    --admin $(provenanced keys show -a validator --home build/run/provenanced --keyring-backend test --testnet) \
    --label msgfees_module_integration_test_v2 \
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto --gas-adjustment 1.2 \
    --fees 359000000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

Execute the contract, sending funds to another account via the contract which assesses fees.

```bash
provenanced tx wasm execute \
    $(provenanced query wasm list-contract-by-code 1 --home build/run/provenanced --testnet -o json | jq -r ".contracts[0]") \
    "{\"send_funds\":{\"funds\":{\"amount\":\"999999\",\"denom\":\"nhash\"},\"to_address\":\"$(provenanced keys show -a receiver --keyring-backend test --home build/run/provenanced -t)\"}}" \
    --amount 999999nhash \
    --from sender \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto --gas-adjustment 1.2 \
    --fees 373000000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

Query the accounts
```bash
provenanced query bank balances $(provenanced keys show -a sender --keyring-backend test --home build/run/provenanced -t) \
    --home build/run/provenanced \
    --testnet
    
provenanced query bank balances $(provenanced keys show -a receiver --keyring-backend test --home build/run/provenanced -t) \
    --home build/run/provenanced \
    --testnet
    
provenanced query bank balances $(provenanced keys show -a feebucket --keyring-backend test --home build/run/provenanced -t) \
    --home build/run/provenanced \
    --testnet
```
