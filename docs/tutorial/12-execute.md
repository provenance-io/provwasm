# Provenance Smart Contract Tutorial

In this section we will execute a purchase by sending funds to the contract instance. The contract
will then perform bank transfers to the merchant and fee collection accounts.

## Execute Contract

To execute a `100purchasecoin` purchase with an ID of `12345`, run using the merchant address from earlier

```bash
provenanced tx wasm execute \
    tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8 \
    '{"purchase":{"id":"12345"}}' \
    --amount 100purchasecoin \
    --from consumer \
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

To ensure the transfers were sent successfully, first query the `merchant` account

```bash
provenanced query bank balances "$merchant" -t -o json | jq
```

This should show that the merchant has increased by `90purchasecoin`

```json
{
  "balances": [
    {
      "denom": "nhash",
      "amount": "200000000000"
    },
    {
      "denom": "purchasecoin",
      "amount": "90"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

Then, query the `feebucket` account

```bash
provenanced query bank balances "$feebucket" -t -o json | jq
```

This should show that the feebucket account has increased by `10purchasecoin`

```json
{
  "balances": [
    {
      "denom": "nhash",
      "amount": "155863718470"
    },
    {
      "denom": "purchasecoin",
      "amount": "10"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

Finally, query the `consumer` account

```bash
provenanced query bank balances "$consumer" -t -o json | jq
```

This should show that it has decreased by `100purchasecoin`

```json
{
  "balances": [
    {
      "denom": "nhash",
      "amount": "199462148015"
    },
    {
      "denom": "purchasecoin",
      "amount": "99900"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

## Up Next

The smart contract has been verified to be deployed and working. This concludes Part 2 of the
tutorial. Proceed to the [Migrate](13-migrate.md) section for information on how to upgrade smart
contracts.
