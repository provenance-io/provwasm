#!/bin/bash -e

# This script stores, instantiates and executes the attrs smart contract
PROV_CMD="provenanced"
WASM="./contracts/attrs/artifacts/attrs.wasm"
declare LOCAL_ARGS
if [ -z "${CI}" ]; then
  PROV_CMD=provenanced
  LOCAL_ARGS="--home build/run/provenanced"
  WASM=$1
fi

export node0=$("$PROV_CMD" keys show -a validator --keyring-backend test --testnet $LOCAL_ARGS)

"$PROV_CMD" tx name bind \
  "sc" \
  "$node0" \
  "pb" \
  --restrict=false \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

"$PROV_CMD" tx wasm store $WASM \
  --instantiate-only-address "$node0" \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

"$PROV_CMD" tx wasm instantiate 1 '{"name": "attrs-itv2.sc.pb"}' \
  --admin="$node0" \
  --label attribute_module_integration_test_v2 \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

# Query for the contract address so we can execute it
export contract=$("$PROV_CMD" query wasm list-contract-by-code 1 -t -o json | jq -r ".contracts[0]")

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"bind_label_name":{}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

"$PROV_CMD" query wasm contract-state smart \
  "$contract" '{"get_label_name":{}}' -t -o json

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"add_label":{"text":"hello"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

# delay to ensure correct order for text1 and text2 below

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"add_label":{"text":"wasm"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

export text1=$("$PROV_CMD" query wasm contract-state smart "$contract" '{"get_labels":{}}' --testnet --output json $LOCAL_ARGS | jq -r ".data.labels[0].text")
export text2=$("$PROV_CMD" query wasm contract-state smart "$contract" '{"get_labels":{}}' --testnet --output json $LOCAL_ARGS | jq -r ".data.labels[1].text")

# we don't know the order that 'text' and 'wasm' could be in so we check both
if [ "$text1" != "hello" ] && [ "$text1" != "wasm" ]; then
  echo "label: '$text1' was not set properly"
  exit 1
fi

if [ "$text2" != "wasm" ] && [ "$text2" != "hello" ]; then
  echo "label: '$text2' was not set properly"
  exit 1
fi

if [ "$text2" == "$text1" ]; then
  echo "$text2 and $text1 are suppose to be 'wasm' and 'hello' not the same"
  exit 1
fi

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"update_label":{"original_text":"hello", "update_text":"goodbye"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

export text1=$("$PROV_CMD" query wasm contract-state smart "$contract" '{"get_labels":{}}' --testnet --output json $LOCAL_ARGS | jq -r ".data.labels[0].text")
export text2=$("$PROV_CMD" query wasm contract-state smart "$contract" '{"get_labels":{}}' --testnet --output json $LOCAL_ARGS | jq -r ".data.labels[1].text")

# we don't know the order that 'text' and 'wasm' could be in so we check both
if [ "$text1" != "goodbye" ] && [ "$text1" != "wasm" ]; then
  echo "label: '$text1' was not set properly to hello"
  exit 1
fi

if [ "$text2" != "wasm" ] && [ "$text2" != "goodbye" ]; then
  echo "label: '$text2' was not set properly to wasm"
  exit 1
fi

if [ "$text2" == "$text1" ]; then
  echo "$text2 and $text1 are suppose to be 'goodbye' and 'wasm' not the same"
  exit 1
fi

"$PROV_CMD" tx wasm execute \
  $contract \
  '{"delete_distinct_label":{"text":"wasm"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

export label_count=$("$PROV_CMD" query wasm contract-state smart "$contract" '{"get_labels":{}}' --testnet --output json $LOCAL_ARGS | jq -r ".data.labels | length")
export text1=$("$PROV_CMD" query wasm contract-state smart "$contract" '{"get_labels":{}}' --testnet --output json $LOCAL_ARGS | jq -r ".data.labels[0].text")

if [ "$label_count" != "1" ]; then
  echo "only 1 label should exist. found: $label_count"
  exit 1
fi

if [ "$text1" = "wasm" ]; then
  echo "label: '$text1' was not distinctly deleted"
  exit 1
fi

"$PROV_CMD" tx wasm execute \
  $contract \
  '{"delete_labels":{}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

export label_count=$("$PROV_CMD" query wasm contract-state smart "$contract" '{"get_labels":{}}' --testnet --output json $LOCAL_ARGS | jq -r ".data.labels | length")

if [ "$label_count" != "0" ]; then
  echo "all labels should be deleted. found: $label_count"
  exit 1
fi

echo "Done with script"
