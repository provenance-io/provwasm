# Trigger Module Integration Test

This a CosmWasm smart contract that tests the Rust bindings and mocks for the provenance
trigger module.

This smart contract tests the following functionality

- Messages
  - Create a block-height based trigger
  - Create a block-time based trigger

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
make run
```

Setup the environment variables and accounts to simplify the `provenanced` command arguments

```bash
# setup all of the necessary keys
export provenanced keys add sender --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
export provenanced keys add receiver --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"

# setup key variables
export node0=$(provenanced keys show -a validator --keyring-backend test --testnet )
export sender=$(provenanced keys show -a sender --keyring-backend test --testnet )
export receiver=$(provenanced keys show -a receiver --keyring-backend test --testnet )

provenanced tx bank send \
  "$node0" \
  "$sender" \
  200000000000nhash \
  --from="$node0" \
  --keyring-backend=test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode=block \
  --yes \
  --testnet \
  --output json
```

Create a root name binding for smart contracts (required once per localnet genesis).

```bash    
provenanced tx name bind \
  "sc" \
  "$node0" \
  "pb" \
  --unrestrict \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet \
  --output json
```

Store the trigger integration test smart contract Wasm in provenance

```bash
provenanced tx wasm store "$WASM" \
  --instantiate-anyof-addresses "$node0" \
  --from="$node0" \
  --keyring-backend="test" \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode=block \
  --yes \
  -t
```

Instantiate the contract

```bash
provenanced tx wasm instantiate 1 '{}' \
  --admin="$node0" \
  --label="trigger" \
  --from="$node0" \
  --keyring-backend="test" \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet
```

## Example Executions

Create a block-height based trigger

```bash  
# Query for the contract address so we can execute it
contract=$("$PROV_CMD" query wasm list-contract-by-code 1 -t -o json | jq -r ".contracts[0]")

# get block height and add delay for trigger
export current_height=$($PROV_CMD q block | jq -r .block.header.height)
export target_height=$((current_height + 30))

provenanced tx wasm execute \
  "$contract" \
  "{
  	\"create_trigger\": {
  		\"event\": {
  			\"block_height_event\": {
  				\"block_height\": \"$target_height\"
  			}
  		},
  		\"to_address\": \"$receiver\"
  	}
  }" \
  --amount 90000nhash \
  --from="$sender" \
  --keyring-backend test \
  --chain-id testing \
  --gas auto \
  --gas-prices="1906nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet \
  --output json
```