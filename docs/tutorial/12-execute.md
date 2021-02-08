# Provenance Smart Contract Tutorial

In this section we will execute a purchase by sending funds to the contract account. The contract
will then peform bank transfers to the merchant and fee collection accounts.

## Execute Contract

To execute a `100pcoin` purchase with an ID of `12345`, run

```bash
provenanced tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"purchase":{"id":"12345"}}' \
    --amount 100pcoin \
    --from consumer \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 4000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
```

To ensure the transfers were sent successfully, first query the `merhcant` account

```bash
provenanced query bank balances \
    $(provenanced keys show -a merchant --home build/node0 --keyring-backend test --testnet) \
    --testnet
```

This should show that the merchant has increased by `90pcoin`

```yaml
balances:
  - amount: "90"
    denom: pcoin
  - amount: "100000"
    denom: nhash
pagination:
  next_key: null
  total: "0"
```

Then, query the `feebucket` account

```bash
provenanced query bank balances \
    $(provenanced keys show -a feebucket --home build/node0 --keyring-backend test --testnet) \
    --testnet
```

This should show that the feebucket account has increased by `10pcoin`

```yaml
balances:
  - amount: "10"
    denom: pcoin
  - amount: "46500"
    denom: nhash
pagination:
  next_key: null
  total: "0"
```

Finally, query the `consumer` account

```bash
provenanced query bank balances \
    $(provenanced keys show -a consumer --home build/node0 --keyring-backend test --testnet) \
    --testnet
```

This should show that it has decreased by `100pcoin`

```yaml
balances:
  - amount: "99900"
    denom: pcoin
  - amount: "92000"
    denom: nhash
pagination:
  next_key: null
  total: "0"
```

## Up Next

The smart contract has been verified to be deployed and working. This concludes Part 2 of the
tutorial. Proceed to the [Migrate](13-migrate.md) section for information on how to upgrade smart
contracts.
