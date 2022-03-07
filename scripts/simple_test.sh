#!/bin/bash -e

mkdir build

# TODO: Download the provenanced binary?

# TODO finish provenance initialization
provenanced init --home=./build

provenanced start --home=./build

# Build the contract
cd ./contracts/tutorial
make all

# return to root directory
cd ../..

# Or... should I use a docker compose so that I am running the provenance testnet in one docker
# container and then storing tests to it?
# No I don't think that would be the best idea...

# setup all of the necessary keys
provenanced keys add merchant --home build/node0 --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
provenanced keys add feebucket --home build/node0 --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
provenanced keys add consumer --home build/node0 --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq

export node0=$(provenanced keys show -a node0 --home build/node0 --keyring-backend test -t)
export merchant=$(provenanced keys show -a merchant --home build/node0 --keyring-backend test -t)
export feebucket=$(provenanced keys show -a feebucket --home build/node0 --keyring-backend test -t)
export consumer=$(provenanced keys show -a consumer --home build/node0 --keyring-backend test -t)

# go ahead and run the contract

provenanced tx wasm store ./contracts/tutorial/artifacts/tutorial.wasm \
    --instantiate-only-address "$feebucket" \
    --from "$feebucket" \
    --keyring-backend test \
    --home build/ \
    --chain-id chain-local \
    --gas auto \
    --gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json | jq

provenanced tx wasm instantiate 1 \
	'{ "contract_name": "tutorial.sc.pb", "purchase_denom": "purchasecoin", "merchant_address": "fixme", "fee_percent": "0.10" }' \
    --admin "$feebucket" \
    --label tutorial \
    --from feebucket \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json | jq

# TODO: I need to get the contract address so that we can put it into the execute below


provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"purchase":{"id":"12345"}}' \
    --amount 100purchasecoin \
    --from consumer \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json | jq

# TODO: I need to parse a json response from the query to verify that I have the correct amount of coins in the consumer, merchant and feebucket accounts



