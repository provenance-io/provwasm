#!/bin/bash -e

set -x

# This script stores, instantiates and executes the marker smart contract
PROV_CMD="provenanced"
WASM="./contracts/name/artifacts/name.wasm"

export node0=$("$PROV_CMD" keys show -a validator --keyring-backend test --testnet )

"$PROV_CMD" tx name bind \
  "sc" \
  "$node0" \
  "pb" \
  --unrestrict \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

"$PROV_CMD" tx wasm store $WASM \
  --instantiate-anyof-addresses "$node0" \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

"$PROV_CMD" tx wasm instantiate 1 '{"name": "name-itv2.sc.pb"}' \
  --admin "$node0" \
  --label name_module_integration_test_v2 \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

# Query for the contract address so we can execute it
export contract=$("$PROV_CMD" query wasm list-contract-by-code 1 --testnet --output json  | jq -r ".contracts[0]")

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"bind_prefix":{"prefix":"nm"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

export address=$("$PROV_CMD" query name resolve "nm.name-itv2.sc.pb" --testnet --output json  | jq -r ".address")

if [ "$address" != "$contract" ] && [ "$address" != "$node0" ]; then
  echo "Wrong address: $address for bound name, should have gotten: $contract"
  exit 1
fi

export resolve=$("$PROV_CMD" query wasm contract-state smart "$contract" '{"resolve":{"name":"name-itv2.sc.pb"}}' -t -o json)

echo "resolve:"
echo "$resolve"

params=$("$PROV_CMD" query wasm contract-state smart "$contract" "{\"params\":{}}" -t -o json)

echo "params:"
echo "$params"

name=$("$PROV_CMD" query wasm contract-state smart "$contract" "{\"lookup\":{\"address\":\"$address\"}}" -t -o json)

echo "lookup:"
echo "$lookup"


"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"unbind_prefix":{"prefix":"nm"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

export query=$("$PROV_CMD" query name resolve "nm.name-itv2.sc.pb" --testnet --output json --log_level="panic" --log_format="json" )

# check to see if the query string starts with `failed` otherwise error because we shouldn't get a name that is unbound
if [[ "$query" == failed* ]]; then
  echo "correctly failed on query after name was bound"
else
  echo "Got name query after name was unbound"
  exit 1
fi

echo "End of name test"
