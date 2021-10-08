# Provenance Smart Contract Tutorial

In this section we will upload the optimized smart contract Wasm to the Provenance Blockchain.

## Store Code

For this tutorial, it is assumed that the fee collection address represents network ownership
(why they get paid fees), thus they must be the uploader of the contract Wasm.

Copy the optimized Wasm file to the Provenance Blockchain project. From the tutorial project dir,
run

```bash
cp artifacts/tutorial.wasm $GOPATH/src/github.com/provenance-io/provenance
```

Then, from the provenance directory, store the optimized smart contract Wasm in the blockchain

```bash
provenanced tx wasm store tutorial.wasm \
    --instantiate-only-address "$feebucket" \
    --from feebucket \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --gas-prices="1905nhash" \
	--gas-adjustment=1.5 \
    --broadcast-mode block \
    --yes \
    --testnet \
	--output json | jq
```

To query the stored code (NOTE: The wasm module query commands only produces JSON output).

```bash
provenanced query wasm list-code -o json | jq
```

Should produce output that resembles (field values may differ) the following.

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "tp102c9nplcvrxmhevc6wenm99q6dfte3k3z8vscv",
      "data_hash": "F2F2CD9AA2C192A95B86E9429BC15DCD6B8859BE54C8C66274B80347D2443D82",
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

The `--instantiate-only-address` flag is important when it is necessary to limit instance creation
to a single account.

Copy the value of the `id` field. It is required to instantiate the contract in the next section.

## Up Next

Proceed to the next section to [Instantiate](10-instantiate.md) the contract.
