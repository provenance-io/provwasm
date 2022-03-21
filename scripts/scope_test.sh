#!/bin/bash -e

# This script stores and instantiates the scope smart contract for the metadata module
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

"$PROV_CMD" tx wasm store ./contracts/scope/artifacts/scope.wasm \
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

"$PROV_CMD" tx wasm instantiate 1 '{"name": "scope-itv2.sc.pb"}' \
    --admin "$node0" \
    --label metadata_module_integration_test_v2 \
    --from="$node0" \
    --keyring-backend test \
    --chain-id="testing" \
    --gas=auto \
	  --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet

echo "done with the scope contract"