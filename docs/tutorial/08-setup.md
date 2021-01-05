# Provenance Smart Contract Tutorial

## Setup

In this section, we will set up everything required to deploy the tutorial smart contract to a
provenance blockchain localnet cluster.

It is assumed that Go 1.15 is installed. See [here](https://golang.org/doc/install) for
installation instructions.

## Blockchain

If not already available, clone the provenance blockchain project.

```bash
mkdir -p $GOPATH/src/github.com/FigureTechnologies
cd $GOPATH/src/github.com/FigureTechnologies
git clone git@github.com:FigureTechnologies/provenance-blockchain.git
```

Navigate to the project checkout the tutorial version tag

```bash
cd ./provenance-blockchain
```

Then, confirm ~/.gitconfig has this setting added.

```bash
[url "git@github.com:"]
    insteadOf = https://github.com/
```

Install commands and start a provenance localnet cluster.

```bash
make install
make localnet-start
```

Once built, the previous ~/.gitconfig entry should be commented out or removed - it is known
to cause Rust build issues, but is initially required to build the blockchain.

## Accounts

Before the WASM can be deployed, accounts need to be set up for the consumer, merchant,
and transfer fee bucket.

Create `merchant` account keys

```bash
provenance keys add merchant --home build/node0/provenance --keyring-backend test
```

Create `feebucket` account keys

```bash
provenance keys add feebucket --home build/node0/provenance --keyring-backend test
```

Create `consumer` account keys

```bash
provenance keys add consumer --home build/node0/provenance --keyring-backend test
```

Fund a `merchant` account with `vspn`, creating it on chain.

```bash
provenance tx send \
    $(provenance keys show -a node0 --home build/node0/provenance --keyring-backend test) \
    $(provenance keys show -a merchant --home build/node0/provenance --keyring-backend test) \
    100000vspn \
    --from node0 \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 2000vspn \
    --broadcast-mode block \
    --yes
```

Fund a `feebucket` account with `vspn`, creating it on chain.

```bash
provenance tx send \
    $(provenance keys show -a node0 --home build/node0/provenance --keyring-backend test) \
    $(provenance keys show -a feebucket --home build/node0/provenance --keyring-backend test) \
    100000vspn \
    --from node0 \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 2000vspn \
    --broadcast-mode block \
    --yes
```

Fund a `consumer` account with `vspn`, creating it on chain.

```bash
provenance tx send \
    $(provenance keys show -a node0 --home build/node0/provenance --keyring-backend test) \
    $(provenance keys show -a consumer --home build/node0/provenance --keyring-backend test) \
    100000vspn \
    --from node0 \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 2000vspn \
    --broadcast-mode block \
    --yes
```

## Name

An unrestricted name, `sc.pb`, must be created so the smart contract can bind a name to its
address.

```bash
provenance tx name bind \
    "sc" \
    $(provenance keys show -a node0 --home build/node0/provenance --keyring-backend test) \
    "pb" \
    --restrict=false \
    --from node0 \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --fees 5000vspn \
    --broadcast-mode block \
    --yes
```

## Marker

A marker must be created in order to mint coins required for purchase transfers.

```bash
provenance tx marker new 1000000000fpcoin \
    --type COIN \
    --from node0 \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 5000vspn \
    --broadcast-mode block \
    --yes
```

Grant withdraw access on the marker to the `node0` marker admin account

```bash
provenance tx marker grant \
    $(provenance keys show -a node0 --home build/node0/provenance --keyring-backend test) \
    fpcoin \
    withdraw \
    --from node0 \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 5000vspn \
    --broadcast-mode block \
    --yes
```

Finalize the marker

```bash
provenance tx marker finalize fpcoin \
    --from node0 \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 5000vspn \
    --broadcast-mode block \
    --yes
```

Activate the marker, minting the `fpcoin` supply

```bash
provenance tx marker activate fpcoin \
    --from node0 \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 5000vspn \
    --broadcast-mode block \
    --yes
```

## Withdraw

Coins must be withdrawn into the `consumer` account in order to send purchase transfers.

```bash
provenance tx marker withdraw fpcoin \
    100000fpcoin \
    $(provenance keys show -a consumer --home build/node0/provenance --keyring-backend test) \
    --from node0 \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 5000vspn \
    --broadcast-mode block \
    --yes
```

The `consumer` account should now have `vspn` to pay network fees, and `fpcoin` for purchases with
the merchant.

```bash
provenance q auth account \
    $(provenance keys show -a consumer --home build/node0/provenance --keyring-backend test)
```

Example account query output

```yaml
address: tp1s2nzrmekarkeckerxnmc6p2pd90yh2spzej0vg
coins:
- denom: fpcoin
  amount: "100000"
- denom: vspn
  amount: "100000"
public_key: ""
account_number: 14
sequence: 0
```

## Up Next

Proceed to the [Store](09-store.md) section to upload the optimized smart contract WASM to the
blockchain.
