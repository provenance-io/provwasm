# Provenance Smart Contract Tutorial

## Setup

In this section, we will set up everything required to deploy the tutorial smart contract to a
Provenance Blockchain localnet cluster.

It is assumed that Go 1.15+ is installed. See [here](https://golang.org/doc/install) for
installation instructions.

## Blockchain

If not already available, clone the Provenance Blockchain project.

```bash
mkdir -p $GOPATH/src/github.com/provenance-io
cd $GOPATH/src/github.com/provenance-io
git clone git@github.com:provenance-io/provenance.git
```

Navigate to the blockchain project and checkout the required branch.

```bash
cd ./provenance
git fetch
git checkout main
```

Install commands and start a provenance localnet cluster.

```bash
make clean
make install
make localnet-start
```

## Accounts

Before the Wasm can be deployed, accounts need to be set up for the consumer, merchant,
and transfer fee bucket.

Create `merchant` account keys

```bash
provenanced keys add merchant --home build/node0 --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
```

Create `feebucket` account keys

```bash
provenanced keys add feebucket --home build/node0 --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
```

Create `consumer` account keys

```bash
provenanced keys add consumer --home build/node0 --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
```

Create alias for the keys
```bash
export node0=$(provenanced keys show -a node0 --home build/node0 --keyring-backend test -t)
export merchant=$(provenanced keys show -a merchant --home build/node0 --keyring-backend test -t)
export feebucket=$(provenanced keys show -a feebucket --home build/node0 --keyring-backend test -t)
export consumer=$(provenanced keys show -a consumer --home build/node0 --keyring-backend test -t)
```

Fund a `merchant` account with `nhash`, creating it on chain.

```bash
provenanced tx bank send \
	"$node0" \
	"$merchant" \
	200000000000nhash \
	--from="$node0" \
	--keyring-backend=test \
	--home=build/node0 \
	--chain-id=chain-local \
	--gas=auto \
	--gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
	--broadcast-mode=block \
	--yes \
	--testnet \
	--output json | jq
```

Fund a `feebucket` account with `nhash`, creating it on chain.

```bash
provenanced tx bank send \
    provenanced tx bank send \
    "$node0" \
    "$feebucket" \
    200000000000nhash \
    --from node0 \
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
```

Fund a `consumer` account with `nhash`, creating it on chain.

```bash
provenanced tx bank send \
    "$node0" \
    "$consumer" \
    200000000000nhash \
    --from node0 \
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
```

## Name

An unrestricted name, `sc.pb`, must be created so the smart contract can bind a name to its
address.

```bash
provenanced tx name bind \
    "sc" \
    "$node0" \
    "pb" \
    --restrict=false \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json | jq
```

## Marker

A marker must be created in order to mint coins required for purchase transfers.

```bash
provenanced tx marker new 1000000000purchasecoin \
    --type COIN \
    --from node0 \
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
```

Grant withdraw access on the marker to the `node0` marker admin account

```bash
provenanced tx marker grant \
    $node0 \
    purchasecoin \
    withdraw \
    --from node0 \
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
```

Finalize the marker

```bash
provenanced tx marker finalize purchasecoin \
    --from node0 \
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
```

Activate the marker, minting the `purchasecoin` supply

```bash
provenanced tx marker activate purchasecoin \
    --from node0 \
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
```

## Withdraw

Coins must be withdrawn into the `consumer` account in order to send purchase transfers.

```bash
provenanced tx marker withdraw purchasecoin \
    100000purchasecoin \
    $consumer \
    --from node0 \
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
```

The `consumer` account should now have `nhash` to pay network fees, and `purchasecoin` for purchases with
the merchant.

```bash
provenanced q bank balances "$consumer" -t -o json | jq
```

Example account query output

```json
{
  "balances": [
    {
      "denom": "nhash",
      "amount": "100000"
    },
    {
      "denom": "purchasecoin",
      "amount": "100000"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

## Up Next

Proceed to the [Store](09-store.md) section to upload the optimized smart contract Wasm to the
blockchain.
