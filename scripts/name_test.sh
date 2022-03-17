#!/bin/bash -e

# This script stores, instantiates and executes the marker smart contract
export PROV_CMD="./bin/provenanced"

export node0=$("$PROV_CMD" keys show -a validator --keyring-backend test --testnet)

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
    --testnet

"$PROV_CMD" tx wasm store name.wasm \
    --instantiate-only-address "$node0" \
    --from="$node0" \
    --keyring-backend test \
    --chain-id="testing" \
    --gas=auto \
	  --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet

"$PROV_CMD" tx wasm instantiate 1 '{"name": "name-itv2.sc.pb"}' \
    --admin "$node0" \
    --label name_module_integration_test_v2 \
    --from="$node0" \
    --keyring-backend test \
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
    '{"bind_prefix":{"prefix":"nm"}}' \
    --from="$node0" \
    --keyring-backend test \
    --chain-id="testing" \
    --gas=auto \
	  --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet

sleep "10s"

echo "$contract"
echo "\n------------------\n"
export address=$("$PROV_CMD" query name resolve "nm.name-itv2.sc.pb" -t -o json | jq -r ".address")

if [ "$address" != "$contract" ]; then
  echo "Wrong address: $address for bound name, should have gotten: $contract"
  exit 1
fi

sleep 10s

"$PROV_CMD" tx wasm execute \
    "$contract" \
    '{"unbind_prefix":{"prefix":"nm"}}' \
    --from="$node0" \
    --keyring-backend test \
    --home build/node0 \
    --chain-id="testing" \
    --gas=auto \
	  --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet

"$PROV_CMD" query name resolve "nm.name-itv2.sc.pb" -t -o json

echo "End of name test"