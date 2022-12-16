#!/usr/bin/env bash
set -x

IBC_REMOTE=../artifacts/ibc_remote.wasm

provenanced tx wasm store $IBC_REMOTE \
    --instantiate-everybody true \
    --from remoteaccount \
    --keyring-backend test \
    --home target \
    --chain-id remote \
    --broadcast-mode block \
    --yes \
    --testnet \
    --gas auto --gas-adjustment 1.5 --fees 6000000000nhash \
    --node tcp://localhost:36657

provenanced tx wasm instantiate 1 '{}' \
    --from remoteaccount \
    --keyring-backend test \
    --home target \
    --chain-id remote \
    --broadcast-mode block \
    --yes \
    --testnet --gas auto --gas-adjustment=2 --fees 5000000000nhash \
   --label test --admin "$(provenanced keys show -a remoteaccount --keyring-backend test --home target --testnet)" \
   --node tcp://localhost:36657

