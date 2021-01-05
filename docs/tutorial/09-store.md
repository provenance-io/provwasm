# Provenance Smart Contract Tutorial

In this section we will upload the optimized smart contract WASM to the provenance blockchain.

## Store Code

For this tutorial, it is assumed that the fee collection address represents network ownership
(why they get paid fees), thus they must be the uploader of the contract WASM.

Copy the optimized WASM file to the `provenance-blockchain` project. From the tutorial project dir,
run

```bash
cp artifacts/tutorial.wasm $GOPATH/src/github.com/FigureTechnologies/provenance-blockchain
```

Then, from the `provenance-blockchain` directory, store the optimized smart contract WASM in
provenance

```bash
provenance tx wasm store tutorial.wasm \
    --source "https://github.com/FigureTechnologies/provwasm/tree/master/contracts/tutorial" \
    --builder "cosmwasm/rust-optimizer:0.10.7" \
    --instantiate-only-address $(provenance keys show -a feebucket --keyring-backend test --home build/node0/provenance) \
    --from feebucket \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 25000vspn \
    --broadcast-mode block \
    --yes
```

To query the stored code (NOTE: The wasm module query commands only produces JSON output).

```bash
provenance query wasm list-code -o json | jq
```

Should produce output that resembles (field values may differ) the following.

```json
[
  {
    "id": 1,
    "creator": "tp1clx2v0ze5wmckerm3wx9c2r7wcaf05ktwyaedj",
    "data_hash": "ACB50FFA4AED7D5230C6ED5D26EA1707A97C1B2B6F84D485DCC8154DD94C5B7A",
    "source": "https://github.com/FigureTechnologies/provwasm/tree/master/contracts/tutorial",
    "builder": "cosmwasm/rust-optimizer:0.10.3"
  }
]
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
