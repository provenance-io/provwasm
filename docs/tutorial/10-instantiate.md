# Provenance Smart Contract Tutorial

In this section we will create an instance of the smart contract.

## Instantiate Contract

To instantiate the contract from the command line, an init message must be created and encoded as
JSON. First, take note of the merchant address by querying for the account address. For example

```bash
provenanced keys show -a merchant --home build/node0 --keyring-backend test --testnet

tp1u3zw79zhzxv8znpkdy35u6npe7aa45szt2ccge
```

Then, create a JSON message using this address with configuration values (formatted here for
readability).

```json
{
  "contract_name": "tutorial-v2.sc.pb",
  "purchase_denom": "purchasecoin",
  "merchant_address": "FIXME",
  "fee_percent": "0.10"
}
```

The contract can then be instantiated using the code ID from the previous section and the
JSON from above. NOTE: Remember that since the `feebucket` represents network ownership, that
account must be the executor (the `--from` flag) of the command. This requirement was further
enforced by the fact that only the `feebucket` account is allowed to instantiate (set with the
`--instantiate-only-address` flag during WASM upload).

```bash
provenanced tx wasm instantiate 1 \
    '{"contract_name":"tutorial-v2.sc.pb","purchase_denom":"purchasecoin","merchant_address":"FIXME","fee_percent":"0.1"}' \
    --admin $(provenanced keys show -a feebucket --home build/node0 --keyring-backend test --testnet) \
    --label tutorial-v2 \
    --from feebucket \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 3500nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

NOTE: Setting the `--admin` account is important. It is impossible to migrate the contract instance
to a new code ID if not set.

The contract can then be queried by code ID.

```bash
provenanced query wasm list-contract-by-code 1 --testnet -o json | jq
```

Should produce output similar to

```json
{
  "contracts": [
    "tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

Copy the value of the contract address field. It is required in later sections to query and execute
the contract instance.

## Up Next

Proceed to the next section to [Query](11-query.md) the contract.
