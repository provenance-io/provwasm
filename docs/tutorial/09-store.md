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
    --instantiate-anyof-addresses "$feebucket" \
    --from "$feebucket" \
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
      "creator": "tp1rj4s08a7va4rdwwk7zmwg90slz956qnjrfa5ta",
      "data_hash": "6B6D1579CF2FF09B197B521E74FDDF09429895FECFA5BDB5D779472FFBCA8CF8",
      "instantiate_permission": {
        "permission": "AnyOfAddresses",
        "address": "",
        "addresses": [
          "tp1rj4s08a7va4rdwwk7zmwg90slz956qnjrfa5ta"
        ]
      }
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

The `--instantiate-anyof-addresses` flag is important when it is necessary to limit instance creation to specified accounts.

Copy the value of the `id` field. It is required to instantiate the contract in the next section.

## Up Next

Proceed to the next section to [Instantiate](10-instantiate.md) the contract.
