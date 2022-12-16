#!/usr/bin/env bash
set -x

IBC_LOCAL_WASM=../artifacts/ibc_local.wasm

provenanced tx wasm store "$IBC_LOCAL_WASM" \
    --instantiate-everybody true \
    --from localaccount \
    --keyring-backend test \
    --home target \
    --chain-id local \
    --broadcast-mode block \
    --yes \
    --testnet --gas auto --gas-adjustment 1.5 --fees 6000000000nhash

provenanced tx wasm instantiate 1 "{}" \
    --from localaccount \
    --keyring-backend test \
    --home target \
    --chain-id local \
    --broadcast-mode block \
    --yes \
    --testnet --gas auto --gas-adjustment=2 --fees 5000000000nhash \
   --label test --admin "$(provenanced keys show -a localaccount --keyring-backend test --home target --testnet)"