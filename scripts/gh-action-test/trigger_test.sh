#!/bin/bash -e

# This script stores, instantiates and executes the msgfees smart contract
PROV_CMD="provenanced"
WASM="./contracts/trigger/artifacts/trigger.wasm"

# setup all of the necessary keys
"$PROV_CMD" keys add sender --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
"$PROV_CMD" keys add receiver --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"

# setup key variables
node0=$("$PROV_CMD" keys show -a validator --keyring-backend test --testnet )
sender=$("$PROV_CMD" keys show -a sender --keyring-backend test --testnet )
receiver=$("$PROV_CMD" keys show -a receiver --keyring-backend test --testnet )

########################################################
# Test setup
########################################################

echo -e "\n\n#########################################################"
echo "Test setup"
echo "#########################################################"

echo "sending coins to different keys"

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

echo "binding name"
# Setup name and new COIN for the smart contract
"$PROV_CMD" tx name bind \
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

echo "storing wasm"
# Run the contract
"$PROV_CMD" tx wasm store "$WASM" \
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

echo "instantiating contract"
"$PROV_CMD" tx wasm instantiate 1 '{}' \
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

# Query for the contract address so we can execute it
contract=$("$PROV_CMD" query wasm list-contract-by-code 1 -t -o json | jq -r ".contracts[0]")

########################################################
# Create block height trigger test
########################################################

echo -e "\n\n#########################################################"
echo "Create block height trigger test"
echo "#########################################################"

# get block height and add delay for trigger
current_height=$($PROV_CMD q block | jq -r .block.header.height)
target_height=$((current_height + 6))

echo "current block height: $current_height"
echo "target block height: $target_height"

"$PROV_CMD" tx wasm execute \
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

triggers=$("$PROV_CMD" query wasm contract-state smart "$contract" "{\"get_trigger\":{}}" -t -o json)
echo "stored triggers:"
echo "$triggers"

sleep 12

# Verify that the funds were sent to the receiver
receiver_query=$("$PROV_CMD" query bank balances "$receiver" --testnet --output json )
receiver_denom=$(echo "$receiver_query" | jq -r ".balances[0].denom")
receiver_amount=$(echo "$receiver_query" | jq -r ".balances[0].amount")

if [ "$receiver_denom" != "nhash" ]; then
  echo "receiver does not have nhash"
  exit 1
fi

if [ "$receiver_amount" != "90000" ]; then
  echo "receiver does not have 90000 coins"
  exit 1
fi

#########################################################
# Create block time trigger test
#########################################################

echo -e "\n\n#########################################################"
echo "Create block time trigger test"
echo "#########################################################"

# get timestamp and add delay for trigger
current_time=$(date +%s)
target_time=$(($current_time + 4))

echo "current time = $current_time"
echo "target time = $target_time"

"$PROV_CMD" tx wasm execute \
  "$contract" \
  "{
  	\"create_trigger\": {
  		\"event\": {
  			\"block_time_event\": {
  				\"timestamp\": \"$target_time\"
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

triggers=$("$PROV_CMD" query wasm contract-state smart "$contract" "{\"get_trigger\":{}}" -t -o json)
echo "stored triggers:"
echo "$triggers"

sleep 8

# Verify that the funds were sent to the receiver
receiver_query=$("$PROV_CMD" query bank balances "$receiver" --testnet --output json )
receiver_amount=$(echo "$receiver_query" | jq -r ".balances[0].amount")

if [ "$receiver_amount" != "180000" ]; then
  echo "receiver does not have 180000 coins"
  exit 1
fi

#########################################################
# Delete trigger test
#########################################################

echo -e "\n\n#########################################################"
echo "Delete trigger test"
echo "#########################################################"

trigger_count=$($PROV_CMD q trigger list all -o json | jq '.triggers | length')

if [ "$trigger_count" != "0" ]; then
  echo "trigger count should be 0"
  exit 1
fi

echo "creating trigger"

current_height=$PROV_CMD q block | jq .block.header.height
"$PROV_CMD" tx wasm execute \
  "$contract" \
  "{
  	\"create_trigger\": {
  		\"event\": {
  			\"block_height_event\": {
  				\"block_height\": \"$((current_height + "600"))\"
  			}
  		},
  		\"to_address\": \"$receiver\"
  	}
  }" \
  --amount 100nhash \
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

trigger_count=$($PROV_CMD q trigger list all -o json | jq '.triggers | length')

if [ "$trigger_count" != "1" ]; then
  echo "failed creating trigger"
  exit 1
fi

echo "deleting trigger"
"$PROV_CMD" tx wasm execute \
  "$contract" \
  "{
  	\"delete_trigger\": {
  		\"id\": \"3\"
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

trigger_count=$($PROV_CMD q trigger list all -o json | jq '.triggers | length')

if [ "$trigger_count" != "0" ]; then
  echo "failed deleting trigger"
  exit 1
fi

triggers=$("$PROV_CMD" query wasm contract-state smart "$contract" "{\"get_trigger\":{}}" -t -o json)
echo "stored triggers:"
echo "$triggers"

echo "Trigger tests successful"
