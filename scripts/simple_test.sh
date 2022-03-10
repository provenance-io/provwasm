#!/bin/bash -e

export Provenance_Version="v1.7.6"

wget "https://github.com/provenance-io/provenance/releases/download/$Provenance_Version/provenance-linux-amd64-v1.7.6.zip"

# this will create a folder with both provenance and libwasm
unzip provenance-linux-amd64-v1.7.6.zip

mkdir ./build

PROV_CMD="./bin/provenanced"
PIO_HOME="./build"
export PIO_HOME

if [ ! -d "$PIO_HOME/config" ]; then
    "$PROV_CMD" -t init --chain-id=testing testing
    "$PROV_CMD" -t keys add validator --keyring-backend test
    "$PROV_CMD" -t add-genesis-root-name validator pio --keyring-backend test
    "$PROV_CMD" -t add-genesis-root-name validator pb --restrict=false \
		--keyring-backend test
    "$PROV_CMD" -t add-genesis-root-name validator io --restrict \
		--keyring-backend test
    "$PROV_CMD" -t add-genesis-root-name validator provenance \
		--keyring-backend test
    "$PROV_CMD" -t add-genesis-account validator 100000000000000000000nhash \
		--keyring-backend test
    "$PROV_CMD" -t gentx validator 1000000000000000nhash \
		--keyring-backend test --chain-id=testing
    "$PROV_CMD" -t add-genesis-marker 100000000000000000000nhash --manager \
		validator --access mint,burn,admin,withdraw,deposit \
		--activate --keyring-backend test
    "$PROV_CMD" -t collect-gentxs
fi
nohup "$PROV_CMD" -t start

# Build the contract
cd ./contracts/tutorial
make all

# return to root directory
cd ../..

# Or... should I use a docker compose so that I am running the provenance testnet in one docker
# container and then storing tests to it?
# No I don't think that would be the best idea...

# setup all of the necessary keys
"$PROV_CMD" keys add merchant --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
"$PROV_CMD" keys add feebucket --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
"$PROV_CMD" keys add consumer --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq

export node0=$("$PROV_CMD" keys show -a node0 --keyring-backend test -t)
export merchant=$("$PROV_CMD" keys show -a merchant --keyring-backend test -t)
export feebucket=$("$PROV_CMD" keys show -a feebucket --keyring-backend test -t)
export consumer=$("$PROV_CMD" keys show -a consumer --keyring-backend test -t)

# Run the contract
"$PROV_CMD" tx wasm store ./contracts/tutorial/artifacts/tutorial.wasm \
    --instantiate-only-address "$feebucket" \
    --from "$feebucket" \
    --keyring-backend test \
    --chain-id chain-local \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json | jq

"$PROV_CMD" tx wasm instantiate 1 \
	'{ "contract_name": "tutorial.sc.pb", "purchase_denom": "purchasecoin", "merchant_address": "fixme", "fee_percent": "0.10" }' \
    --admin "$feebucket" \
    --label tutorial \
    --from feebucket \
    --keyring-backend test \
    --chain-id chain-local \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json | jq

# TODO: I need to get the contract address so that we can put it into the execute below


"$PROV_CMD" tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"purchase":{"id":"12345"}}' \
    --amount 100purchasecoin \
    --from consumer \
    --keyring-backend test \
    --chain-id chain-local \
    --gas auto \
    --gas-prices="1905nhash" \
	  --gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	  --output json | jq

# TODO: I need to parse a json response from the query to verify that I have the correct amount of coins in the consumer, merchant and feebucket accounts

# Check out ATS README.md
# What all do we need for inputs and outputs if this is a github action?
# Actual actions in the github actions repo hub so other smart contract users can have.

# 1. Tutorial tests
# 2. All contracts
# 3. Github action for 3rd party actions
# 4. Document action for external use
# 5. Integrating with ATS and other Figure Smart Contracts







