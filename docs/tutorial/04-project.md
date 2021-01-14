# Provenance Smart Contract Tutorial

## Generate Project

For the time being, we need to develop within the `provwasm` project. Once it is published
to [crates.io](https://crates.io), this restriction will no longer apply.

```bash
git clone git@github.com:provenance-io/provwasm.git
```

After cloning the project, navigate to the contracts directory

```bash
cd ./provwasm/contracts
```

CosmWasm has a template repo that can be used as a starting point for development. To generate the
tutorial project, run

```bash
cargo generate --git https://github.com/CosmWasm/cosmwasm-template.git --name tutorial
```

Navigate to the project

```bash
cd ./tutorial
```

Edit Cargo.toml, changing the `exclude` list to the following

```toml
exclude = [
  "tutorial.wasm",
  "checksums.txt",
]
```

Set contract dependencies

```toml
[dependencies]
provwasm-std = { path = "../../packages/bindings", version = "0.13.2" }
cosmwasm-std = { version = "0.13.2" }
cosmwasm-storage = { version = "0.13.2" }
schemars = "0.7"
serde = { version = "1.0.103", default-features = false, features = ["derive"] }
thiserror = { version = "1.0.21" }

[dev-dependencies]
provwasm-mocks = { path = "../../packages/mocks", version = "0.13.2" }
cosmwasm-schema = { version = "0.13.2" }
```

Reset the README and clear out the current JSON schema artifacts.

```bash
echo '# Purchase Smart Contact' > README.md
rm -rf schema/*.json
```

## Up Next

Proceed to the [Requirements](05-requirements.md) section to define smart contract functionality.
