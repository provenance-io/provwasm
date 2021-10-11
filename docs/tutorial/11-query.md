# Provenance Smart Contract Tutorial

In this section we will query the contract for configuration state.

## Query Contract

To query the contract instance from the command line, run

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz '{"query_request":{}}' -t -o json | jq
```

NOTE: Even though, the query request in this tutorial does not take parameters, a body must still
be provided as the empty JSON object: `{}`

This should produce output similar to the following

```json
{
  "data": {
    "purchase_denom": "purchasecoin",
    "merchant_address": "tp1jeqf9m9psa5jvurzpwtdk5m5429fhq48f0u5wq",
    "fee_collection_address": "tp102c9nplcvrxmhevc6wenm99q6dfte3k3z8vscv",
    "fee_percent": "0.1"
  }
}
```

Nothing too complex here. Other contracts will most likely require multiple types of query requests.

## Up Next

Proceed to the next section to [Execute](12-execute.md) the contract.
