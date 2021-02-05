## Bilateral Exchange Smart Contract

This a CosmWasm smart contract that provides the bilateral exchange of `provenance` `markers`.

It binds a `provenance` `name` on initialization and deletes contract data and `name` binding upon successful execution or contract cancellation.

This smart contract provides the following functionality:

- Messages
  - Initialize contract with `collateral` and `ask`
  - Bind contract name to contract address
  - Offer `marker` in exchange for `collateral`
  - Cancel contract
- Queries
  - Get contract state data

## Build

Set the install location for the compiled wasm, for example

```bash
export PROVWASM_INSTALL_DIR="$PIO_HOME/artifacts"
```

Compile and install

```bash
make && make install
```

## Example Usage

_NOTE: Address bech32 values and other params may vary._

_Optionally create an unrestricted name binding parent for smart-contracts_
```bash
provenanced tx name bind \
    "sc" \
    (provenanced keys show -a node0 --home build/node0 --keyring-backend test --testnet) \
    "pb" \
    --restrict=false \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

1. Store the `bilateral-exchange` WASM.

```bash
provenanced tx wasm store bilateral_exchange.wasm \
    --source "https://github.com/provenance-io/provwasm/tree/main/contracts/bilateral-exchange" \
    --builder "cosmwasm/rust-optimizer:0.10.7" \
    --from node0 \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 25000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

2. Instantiate the contract, binding the name `bilateral-ex.sc.pb` to the contract address.

_NOTE: Replace `M1_AMT` and `M1_DENOM` with the `ask` marker. Replace `M2` with the `collateral` marker._

```bash
provenanced tx wasm instantiate 1 \
    '{"name":"bilateral-ex.sc.pb","ask":[{"amount":"M1_AMT","denom":"M1_DENOM"}]}' \
    --admin (provenanced keys show -a seller --home build/node0 --keyring-backend test --testnet) \
    --from seller \
    --amount MARK \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --label bilateral-gme \
    --gas auto \
    --gas-adjustment 1.4 \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

3. Execute the contract by sending an `offer` marker, matching the `ask` marker.

_NOTE: Replace `M1` with the `offer` marker._

_NOTE++: The json data '{"offer":{}}' represents the action and additional data to pass into the smart contract, not the actual marker being sent as an offer. That is the `--amount` option._

```bash
provenanced tx wasm execute \
  tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
  '{"offer":{}}' \
  --amount M1 \
  --from buyer \
  --keyring-backend test \
  --home build/node0 \
  --chain-id chain-local \
  --gas auto \
  --gas-adjustment 1.4 \
  --fees 5000nhash \
  --broadcast-mode block \
  --yes \
  --testnet | jq
```

## Other actions

Cancel the contract.

```bash
provenanced tx wasm execute \
  tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
  '{"cancel":{}}' \
  --from seller \
  --keyring-backend test \
  --home build/node0 \
  --chain-id chain-local \
  --gas auto \
  --gas-adjustment 1.4 \
  --fees 5000nhash \
  --broadcast-mode block \
  --yes \
  --testnet | jq
```

Query the state of the contract.

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz '{"query_state":{}}' --testnet
```