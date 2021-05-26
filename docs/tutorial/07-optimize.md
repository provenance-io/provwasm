# Provenance Smart Contract Tutorial

## Optimize

In this section we will optimize the compiled smart contract Wasm to a deployable file.

A rust optimization tool was developed by the CosmWasm team to reduce the size of smart contract
Wasm. It is packaged as a docker image. To use this image, add the following to the end of the
tutorial `Makefile`.

```Makefile
.PHONY: optimize
optimize:
	@docker run --rm -v $(CURDIR):/code \
		--mount type=volume,source=tutorial_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.11.3
```

Then build the optimized Wasm

```bash
make optimize
```

There should now be an `artifacts` directory with the following contents

```bash
ls -lh artifacts/tutorial.wasm

-rw-r--r-- 1 user user 160K Dec 17 10:54 artifacts/tutorial.wasm
```

The optimized Wasm file should be significantly smaller than the original.

```bash
ls -lh target/wasm32-unknown-unknown/release/tutorial.wasm

-rwxrwxr-x 2 user user 1.7M Dec 17 10:53 target/wasm32-unknown-unknown/release/tutorial.wasm
```

NOTE: Optimized smart contract size must be smaller than `600K`

This concludes Part 1 of the tutorial. The optimized smart contract Wasm is ready to deploy to
the Provenance Blockchain.

## Up Next

Proceed to the [Setup](08-setup.md) section to begin Part 2 of the tutorial.
