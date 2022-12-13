#!/usr/bin/env bash
set -x

hermes --config config.toml keys add --chain local --mnemonic-file local-relayer-mnemonic --hd-path "m/44'/1'/0'/0/0"
hermes --config config.toml keys add --chain remote --mnemonic-file remote-relayer-mnemonic --hd-path "m/44'/1'/0'/0/0"

# on local chain
LOCAL_CONTRACT_ADDRESS=tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8
# on remote chain
REMOTE_CONTRACT_ADDRESS=tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8
# the version of the contracts common api
CHANNEL_VERSION=pio-ibc-example-v1
#chain ids
LOCAL_CHAIN_ID=local
REMOTE_CHAIN_ID=remote

hermes --config config.toml create channel \
--a-chain $LOCAL_CHAIN_ID \
--b-chain $REMOTE_CHAIN_ID \
--a-port wasm.$LOCAL_CONTRACT_ADDRESS \
--b-port wasm.$REMOTE_CONTRACT_ADDRESS \
--order ordered \
--channel-version $CHANNEL_VERSION \
--new-client-connection \
--yes

hermes --config config.toml start
