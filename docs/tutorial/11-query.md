# Provenance Smart Contract Tutorial

In this section we will query the contract for configuration state.

## Query Contract

To query the contract instance from the command line, run

```bash
provenance query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz '{"query_request":{}}' | jq
```

NOTE: Even though, the query request in this tutorial does not take parameters, a body must still
be provided as the empty JSON object: `{}`

This should produce output similar to the following

```json
{
  "contract_name": "tutorial-v2.sc.pb",
  "purchase_denom": "fpcoin",
  "merchant_address": "tp168zfahluza55e2vxrdta0c5c0dx5asdpvlz7pw",
  "fee_collection_address": "tp1clx2v0ze5wmckerm3wx9c2r7wcaf05ktwyaedj",
  "fee_percent": "0.1"
}
```

Nothing too complex here. Other contracts will most likely require multiple types of query requests.

## Up Next

Proceed to the next section to [Execute](12-execute.md) the contract.
