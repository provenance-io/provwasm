# Provenance Smart Contract Tutorial

In this section we will query the contract for configuration state.

## Query Contract

To query the contract instance from the command line, run

```bash
provenanced query wasm contract-state smart \
    tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8 '{"query_request":{}}' -t -o json | jq
```

NOTE: Even though, the query request in this tutorial does not take parameters, a body must still
be provided as the empty JSON object: `{}`

This should produce output similar to the following

```json
{
  "data": {
    "purchase_denom": "purchasecoin",
    "merchant_address": "tp1p00sxn3yqdnwp8v60watjw35k0cn25gnamkguh",
    "fee_collection_address": "tp1rj4s08a7va4rdwwk7zmwg90slz956qnjrfa5ta",
    "fee_percent": "0.1"
  }
}
```

Nothing too complex here. Other contracts will most likely require multiple types of query requests.

## Up Next

Proceed to the next section to [Execute](12-execute.md) the contract.
