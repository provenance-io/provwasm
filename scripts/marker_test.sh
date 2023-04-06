#!/bin/bash -e

# This script stores, instantiates and executes the marker smart contract
PROV_CMD="provenanced"
WASM="./contracts/marker/artifacts/marker.wasm"
declare LOCAL_ARGS
if [ -z "${CI}" ]; then
  PROV_CMD=provenanced
  LOCAL_ARGS="--home build/run/provenanced"
  WASM=$1
fi

export node0=$("$PROV_CMD" keys show -a validator --keyring-backend test --testnet $LOCAL_ARGS)

"$PROV_CMD" tx name bind \
  "sc" \
  $"$node0" \
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

"$PROV_CMD" tx wasm instantiate 1 '{"name":"marker-itv2.sc.pb"}' \
  --admin "$node0" \
  --label marker_module_integration_test_v2 \
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
  '{"create":{"supply":"500","denom":"faustiancoin","allow_forced_transfer":false}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"create":{"supply":"10","denom":"forcedtransfercoin","allow_forced_transfer":true}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

"$PROV_CMD" q marker list --testnet
#"$PROV_CMD" q wasm contract-state smart "$contract" '{"get_by_denom":{"denom":"faustiancoin"}}' --testnet -o json

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"grant_access":{"denom":"faustiancoin"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"finalize":{"denom":"faustiancoin"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"activate":{"denom":"faustiancoin"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"withdraw":{"amount":"400","denom":"faustiancoin"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

"$PROV_CMD" tx wasm execute \
  "$contract" \
  "{\"transfer\":{\"amount\":\"200\",\"denom\":\"faustiancoin\",\"to\":\"$node0\"}}" \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

# verify that the amount was withdrawn into the node0 address
export faustiancoin=$("$PROV_CMD" q bank balances "$node0" --testnet --output json $LOCAL_ARGS | jq -r ".balances[0].amount")

if [ "$faustiancoin" != "200" ]; then
  echo "We failed to get 200faustiancoin and instead got: $faustiancoin"
  exit 1
fi

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"mint":{"amount":"100","denom":"faustiancoin"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

"$PROV_CMD" tx wasm execute \
  "$contract" \
  '{"burn":{"amount":"200","denom":"faustiancoin"}}' \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet $LOCAL_ARGS

echo "Finished marker test script"
