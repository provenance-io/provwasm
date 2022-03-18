#!/bin/bash -e

# This script stores, instantiates and executes the tutorial smart contract
PROV_CMD="./bin/provenanced"

# setup all of the necessary keys
"$PROV_CMD" keys add merchant --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
"$PROV_CMD" keys add feebucket --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
"$PROV_CMD" keys add consumer --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"

echo "sleeping after adding keys"
sleep 10s

# setup key variables
export node0=$("$PROV_CMD" keys show -a validator --keyring-backend test -t)
export merchant=$("$PROV_CMD" keys show -a merchant --keyring-backend test -t)
export feebucket=$("$PROV_CMD" keys show -a feebucket --keyring-backend test -t)
export consumer=$("$PROV_CMD" keys show -a consumer --keyring-backend test -t)

echo "Sending coins to different keys"

"$PROV_CMD" tx bank send \
	"$node0" \
	"$merchant" \
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

"$PROV_CMD" tx bank send \
	"$node0" \
	"$consumer" \
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

"$PROV_CMD" tx bank send \
	"$node0" \
	"$feebucket" \
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

"$PROV_CMD" tx marker new 1000000000purchasecoin \
    --type COIN \
    --from="$node0" \
    --keyring-backend test \
    --chain-id="testing" \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json

"$PROV_CMD" tx marker grant \
    $node0 \
    purchasecoin \
    withdraw \
    --from="$node0" \
    --keyring-backend test \
    --chain-id="testing" \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json

"$PROV_CMD" tx marker finalize purchasecoin \
    --from="$node0" \
    --keyring-backend test \
    --chain-id="testing" \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json

"$PROV_CMD" tx marker activate purchasecoin \
    --from="$node0" \
    --keyring-backend test \
    --chain-id="testing" \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json

"$PROV_CMD" tx marker withdraw purchasecoin \
    100000purchasecoin \
    $consumer \
    --from="$node0" \
    --keyring-backend test \
    --chain-id="testing" \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json

current_dir=pwd
ls

cd \
ls

# Run the contract
"$PROV_CMD" tx wasm store provwasm_tutorial.wasm \
    --instantiate-only-address "$feebucket" \
    --from "$feebucket" \
    --keyring-backend="test" \
    --chain-id="testing" \
    --gas=auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode=block \
    --yes \
    -t

cd $current_dir

# create the json for instantiating the contract with our merchant address
export json="{ \"contract_name\": \"tutorial.sc.pb\", \"purchase_denom\": \"purchasecoin\", \"merchant_address\": \"$merchant\", \"fee_percent\": \"0.10\" }"

"$PROV_CMD" tx wasm instantiate 1 "$json" \
    --admin="$feebucket" \
    --label="tutorial" \
    --from="$feebucket" \
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
    '{"purchase":{"id":"12345"}}' \
    --amount 100purchasecoin \
    --from="$consumer" \
    --keyring-backend test \
    --chain-id testing \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json

# Verify that the funds were sent to the correct accounts for the merchant and the feebucket
export merchant_query=$("$PROV_CMD" query bank balances "$merchant" -t -o json)
export merchant_denom=$(echo "$merchant_query" | jq -r ".balances[1].denom")
export merchant_amount=$(echo "$merchant_query" | jq -r ".balances[1].amount")

if [ "$merchant_denom" != "purchasecoin" ]; then
  exit 1
fi

if [ "$merchant_amount" != "90" ]; then
  exit 1
fi

export feebucket_query=$("$PROV_CMD" query bank balances "$feebucket" -t -o json)
export feebucket_denom=$(echo "$feebucket_query" | jq -r ".balances[1].denom")
export feebucket_amount=$(echo "$feebucket_query" | jq -r ".balances[1].amount")

if [ "$feebucket_denom" != "purchasecoin" ]; then
  exit 1
fi

if [ "$feebucket_amount" != "10" ]; then
  exit 1
fi

echo "All good!"