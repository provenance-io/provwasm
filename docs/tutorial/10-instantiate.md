# Provenance Smart Contract Tutorial

In this section we will create an instance of the smart contract.

## Instantiate Contract

To instantiate the contract from the command line, an init message must be created and encoded as
JSON. First, take note of the merchant address by querying for the account address. For example

```bash
provenanced keys show -a merchant --home build/run/provenanced --keyring-backend test --testnet

tp1p00sxn3yqdnwp8v60watjw35k0cn25gnamkguh
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
`--instantiate-only-address` flag during Wasm upload).

```bash
provenanced tx wasm instantiate 1 \
	'{ "contract_name": "tutorial.sc.pb", "purchase_denom": "purchasecoin", "merchant_address": "fixme", "fee_percent": "0.10" }' \
    --admin "$feebucket" \
    --label tutorial \
    --from feebucket \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto \
    --gas-prices="100000nhash" \
	--gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json | jq
```

NOTE: Setting the `--admin` account is important. It is impossible to migrate the contract instance
to a new code ID if not set.

The contract can then be queried by code ID.

```bash
provenanced query wasm list-contract-by-code 1 -t -o json | jq
```

Should produce output similar to

```json
{
  "contracts": [
    "tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8"
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
