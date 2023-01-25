# Provenance Smart Contract Tutorial

## Setup

In this section, we will set up everything required to deploy the tutorial smart contract to a
Provenance Blockchain localnet cluster.

It is assumed that Go 1.18+ is installed. See [here](https://golang.org/doc/install) for installation instructions.

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
make clean build install
make run
```

## Accounts

Before the Wasm can be deployed, accounts need to be set up for the consumer, merchant,
and transfer fee bucket.

Create `merchant` account keys

```bash
provenanced keys add merchant --home build/run/provenanced --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
```

Create `feebucket` account keys

```bash
provenanced keys add feebucket --home build/run/provenanced --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
```

Create `consumer` account keys

```bash
provenanced keys add consumer --home build/run/provenanced --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0" --output json | jq
```

Create alias for the keys
```bash
export validator=$(provenanced keys show -a validator --home build/run/provenanced --keyring-backend test -t)
export merchant=$(provenanced keys show -a merchant --home build/run/provenanced --keyring-backend test -t)
export feebucket=$(provenanced keys show -a feebucket --home build/run/provenanced --keyring-backend test -t)
export consumer=$(provenanced keys show -a consumer --home build/run/provenanced --keyring-backend test -t)
```

Fund a `merchant` account with `nhash`, creating it on chain.

```bash
provenanced tx bank send \
	"$validator" \
	"$merchant" \
	200000000000nhash \
	--from="$validator" \
	--keyring-backend=test \
	--home=build/run/provenanced \
	--chain-id=testing \
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
	"$validator" \
	"$feebucket" \
	200000000000nhash \
	--from="$validator" \
	--keyring-backend=test \
	--home=build/run/provenanced \
	--chain-id=testing \
	--gas=auto \
	--gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
	--broadcast-mode=block \
	--yes \
	--testnet \
	--output json | jq
```

Fund a `consumer` account with `nhash`, creating it on chain.

```bash
provenanced tx bank send \
    "$validator" \
    "$consumer" \
    200000000000nhash \
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
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
    "$validator" \
    "pb" \
    --unrestrict \
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas-prices="100000nhash" \
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
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto \
    --gas-prices="1000000nhash" \
	--gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json | jq
```

Grant withdraw access on the marker to the `validator` marker admin account

```bash
provenanced tx marker grant \
    $validator \
    purchasecoin \
    withdraw \
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
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
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
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
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
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
    --from validator \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
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
      "amount": "200000000000"
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
