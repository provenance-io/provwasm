# Provenance Smart Contract Tutorial

In this section we will query the contract for configuration state.

## Query Contract

To query the contract instance from the command line, run

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz '{"query_request":{}}' --testnet
```

NOTE: Even though, the query request in this tutorial does not take parameters, a body must still
be provided as the empty JSON object: `{}`

This should produce output similar to the following

```yaml
data:
  contract_name: tutorial-v2.sc.pb
  fee_collection_address: tp18ef6kll9ffpz06ergm6v9xyqtn7pzg9vux8e0z
  fee_percent: "0.1"
  merchant_address: tp1u3zw79zhzxv8znpkdy35u6npe7aa45szt2ccge
  purchase_denom: pcoin
```

Nothing too complex here. Other contracts will most likely require multiple types of query requests.

## Up Next

Proceed to the next section to [Execute](12-execute.md) the contract.
