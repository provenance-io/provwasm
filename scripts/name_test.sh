#!/bin/bash -e

# This script stores, instantiates and executes the marker smart contract
export PROV_CMD="./bin/provenanced"
PROV_CMD="./bin/provenanced"
WASM="./contracts/name/artifacts/name.wasm"
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
    --testnet $LOCAL_ARGS

# Query for the contract address so we can execute it
export contract=$("$PROV_CMD" query wasm list-contract-by-code 1 --testnet --output json $LOCAL_ARGS | jq -r ".contracts[0]")

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
    --testnet $LOCAL_ARGS

export address=$("$PROV_CMD" query name resolve "nm.name-itv2.sc.pb" --testnet --output json $LOCAL_ARGS | jq -r ".address")

if [ "$address" != "$contract" ] && [ "$address" != "$node0" ]; then
  echo "Wrong address: $address for bound name, should have gotten: $contract"
  exit 1
fi

"$PROV_CMD" tx wasm execute \
    "$contract" \
    '{"unbind_prefix":{"prefix":"nm"}}' \
    --from="$node0" \
    --keyring-backend test \
    --chain-id="testing" \
    --gas=auto \
	  --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet $LOCAL_ARGS

export query=$("$PROV_CMD" query name resolve "nm.name-itv2.sc.pb" --testnet --output json --log_level="panic" --log_format="json" $LOCAL_ARGS)

# check to see if the query string starts with `failed` otherwise error because we shouldn't get a name that is unbound
if [[ "$query" == failed* ]]; then
  echo "correctly failed on query after name was bound"
else
  echo "Got name query after name was unbound"
  exit 1
fi

echo "End of name test"