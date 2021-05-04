# Provenance Smart Contract Tutorial

In this section we will upload the optimized smart contract WASM to the Provenance Blockchain.

## Store Code

For this tutorial, it is assumed that the fee collection address represents network ownership
(why they get paid fees), thus they must be the uploader of the contract WASM.

Copy the optimized WASM file to the Provenance Blockchain project. From the tutorial project dir,
run

```bash
cp artifacts/tutorial.wasm $GOPATH/src/github.com/provenance-io/provenance
```

Then, from the provenance directory, store the optimized smart contract WASM in the blockchain

```bash
provenanced tx wasm store tutorial.wasm \
    --source "https://github.com/provenance-io/provwasm/tree/main/contracts/tutorial" \
    --builder "cosmwasm/rust-optimizer:0.11.0" \
    --instantiate-only-address $(provenanced keys show -a feebucket --keyring-backend test --home build/node0 --testnet) \
    --from feebucket \
    --keyring-backend test \
    --home build/node0 \
    --chain-id chain-local \
    --gas auto \
    --fees 25000nhash \
    --broadcast-mode block \
    --yes \
    --testnet
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
      "id": 1,
      "creator": "tp18ef6kll9ffpz06ergm6v9xyqtn7pzg9vux8e0z",
      "data_hash": "9BAE475D812850DF789ACB8252582FDCFB6627593BC44234D75F6002E48DFFD5",
      "source": "https://github.com/provenance-io/provwasm/tree/main/contracts/tutorial",
      "builder": "cosmwasm/rust-optimizer:0.11.0"
    }
  ],
  "pagination": {}
}
```

When storing contracts, it is important to set the `--source` and `--builder` fields. 3rd parties
should be able to download the source (usually an archive file) and create a release WASM file with
the listed builder. They can then compare the checksum of the generated file to the `data_hash`
above. If the hashes match, the source listed is verified to have produced the WASM deployed to the
blockchain.

The `--instantiate-only-address` flag is important when it is necessary to limit instance creation
to a single account.

Copy the value of the `id` field. It is required to instantiate the contract in the next section.

## Up Next

Proceed to the next section to [Instantiate](10-instantiate.md) the contract.
