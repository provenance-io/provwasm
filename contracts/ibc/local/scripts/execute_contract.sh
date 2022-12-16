#!/usr/bin/env bash
set -x

CONTRACT_ADDR=$(provenanced query wasm list-contract-by-code 1 --home target --testnet -o json | jq -r ".contracts[0]");

# be sure the channel is set correctly
provenanced tx wasm execute "$CONTRACT_ADDR" '{"who_am_i":{"channel_id":"channel-0"}}' \
    --from localaccount \
    --keyring-backend test \
    --home target \
    --chain-id local \
    --broadcast-mode block \
    --yes \
    --testnet --gas auto --gas-adjustment=2 --fees 5000000000nhash

watch "provenanced query wasm contract-state smart $CONTRACT_ADDR '{\"list_accounts\":{}}' -t -o json --home target | jq"