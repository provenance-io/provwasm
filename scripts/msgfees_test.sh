#!/bin/bash -e

# This script stores, instantiates and executes the msgfees smart contract
PROV_CMD="./bin/provenanced"

# setup all of the necessary keys
"$PROV_CMD" keys add sender --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
"$PROV_CMD" keys add feebucket --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
"$PROV_CMD" keys add receiver --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"

echo "sleeping after adding keys"
sleep 10s

# setup key variables
export node0=$("$PROV_CMD" keys show -a validator --keyring-backend test -t)
export sender=$("$PROV_CMD" keys show -a sender --keyring-backend test -t)
export feebucket=$("$PROV_CMD" keys show -a feebucket --keyring-backend test -t)
export receiver=$("$PROV_CMD" keys show -a receiver --keyring-backend test -t)

echo "Sending coins to different keys"

"$PROV_CMD" tx bank send \
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

echo "Sleeping to allow send txs to process"
sleep 10s

# Setup name and new COIN for the smart contract
"$PROV_CMD" tx name bind \
    "sc" \
    "$node0" \
    "pb" \
    --restrict=false \
    --from="$node0" \
    --keyring-backend test \
    --chain-id="testing" \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json

# Run the contract
"$PROV_CMD" tx wasm store ./contracts/msgfees/artifacts/msgfees.wasm \
    --instantiate-only-address "$node0" \
    --from "$node0" \
    --keyring-backend="test" \
    --chain-id="testing" \
    --gas=auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode=block \
    --yes \
    -t

"$PROV_CMD" tx wasm instantiate 1 '{"fee_amount":{"amount":"10000","denom":"nhash"},"fee_recipient":"$feebucket"}' \
    --admin="$node0" \
    --label="msgfees" \
    --from="$node0" \
    --keyring-backend="test" \
    --chain-id="testing" \
    --gas=auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet

# Query for the contract address so we can execute it
export contract=$("$PROV_CMD" query wasm list-contract-by-code 1 -t -o json | jq -r ".contracts[0]")

"$PROV_CMD" tx wasm execute \
    "$contract" \
    '{"send_funds":{"funds":{"amount":"999999","denom":"nhash"},"to_address":"$receiver"}}' \
    --amount 999999nhash \
    --from="$sender" \
    --keyring-backend test \
    --chain-id testing \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json

# Verify that the funds were sent to the correct accounts for the receiver and the feebucket
export receiver_query=$("$PROV_CMD" query bank balances "$receiver" -t -o json)
export receiver_denom=$(echo "$receiver_query" | jq -r ".balances[1].denom")
export receiver_amount=$(echo "$receiver_query" | jq -r ".balances[1].amount")

if [ "$receiver_denom" != "nhash" ]; then
  exit 1
fi

if [ "$receiver_amount" != "90" ]; then
  exit 1
fi

export feebucket_query=$("$PROV_CMD" query bank balances "$feebucket" -t -o json)
export feebucket_denom=$(echo "$feebucket_query" | jq -r ".balances[1].denom")
export feebucket_amount=$(echo "$feebucket_query" | jq -r ".balances[1].amount")

if [ "$feebucket_denom" != "nhash" ]; then
  exit 1
fi

if [ "$feebucket_amount" != "10" ]; then
  exit 1
fi

echo "All good!"