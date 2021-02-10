# Provenance Smart Contract Tutorial

## Setup

In this section, we will set up everything required to deploy the tutorial smart contract to a
Provenance Blockchain localnet cluster.

It is assumed that Go 1.15 is installed. See [here](https://golang.org/doc/install) for
installation instructions.

## Blockchain

If not already available, clone the Provenance Blockchain project.

```bash
mkdir -p $GOPATH/src/github.com/provenance-io
cd $GOPATH/src/github.com/provenance-io
git clone git@github.com:provenance-io/provenance.git
```

Navigate to the blockchain project

```bash
cd ./provenance
```

Install commands and start a provenance localnet cluster.

```bash
make install
make localnet-start
```

## Accounts

Before the WASM can be deployed, accounts need to be set up for the consumer, merchant,
and transfer fee bucket.

Create `merchant` account keys

```bash
provenanced keys add merchant --home build/node0 --keyring-backend test --testnet
```

Create `feebucket` account keys

```bash
provenanced keys add feebucket --home build/node0 --keyring-backend test --testnet
```

Create `consumer` account keys

```bash
provenanced keys add consumer --home build/node0 --keyring-backend test --testnet
```

Fund a `merchant` account with `nhash`, creating it on chain.

```bash
provenanced tx bank send \
    $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    $(provenanced keys show -a merchant --home build/node0 --keyring-backend test --testnet) \
    100000nhash \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 2000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

Fund a `feebucket` account with `nhash`, creating it on chain.

```bash
provenanced tx bank send \
    $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    $(provenanced keys show -a feebucket --home build/node0 --keyring-backend test --testnet) \
    100000nhash \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 2000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

Fund a `consumer` account with `nhash`, creating it on chain.

```bash
provenanced tx bank send \
    $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    $(provenanced keys show -a consumer --home build/node0 --keyring-backend test --testnet) \
    100000nhash \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 2000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

## Name

An unrestricted name, `sc.pb`, must be created so the smart contract can bind a name to its
address.

```bash
provenanced tx name bind \
    "sc" \
    $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    "pb" \
    --restrict=false \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

## Marker

A marker must be created in order to mint coins required for purchase transfers.

```bash
provenanced tx marker new 1000000000pcoin \
    --type COIN \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

Grant withdraw access on the marker to the `node0` marker admin account

```bash
provenanced tx marker grant \
    $(provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    pcoin \
    withdraw \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

Finalize the marker

```bash
provenanced tx marker finalize pcoin \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

Activate the marker, minting the `pcoin` supply

```bash
provenanced tx marker activate pcoin \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

## Withdraw

Coins must be withdrawn into the `consumer` account in order to send purchase transfers.

```bash
provenanced tx marker withdraw pcoin \
    100000pcoin \
    $(provenanced keys show -a consumer --home build/node0 --keyring-backend test --testnet) \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

The `consumer` account should now have `nhash` to pay network fees, and `pcoin` for purchases with
the merchant.

```bash
provenanced q bank balances \
    $(provenanced keys show -a consumer --home build/node0 --keyring-backend test --testnet) \
    --testnet
```

Example account query output

```yaml
balances:
  - amount: "100000"
    denom: pcoin
  - amount: "100000"
    denom: nhash
pagination:
  next_key: null
  total: "0"
```

## Up Next

Proceed to the [Store](09-store.md) section to upload the optimized smart contract WASM to the
blockchain.
