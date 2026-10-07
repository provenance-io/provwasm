# Provenance Smart Contract Tutorial

The smart contract created in this tutorial is located in the `contracts/tutorial` folder.

## Generate Project

CosmWasm has a template repo that can be used as a starting point for development. To generate the
tutorial project, run

```bash
cargo generate --git https://github.com/CosmWasm/cw-template.git --name tutorial
```

Navigate to the project

```bash
cd ./tutorial
```

Edit Cargo.toml to have the following contract dependencies

```toml
[dependencies]
cosmwasm-schema = "3.0.10"
cosmwasm-std = { version = "3.0.10", features = ["exports"] }
cw-storage-plus = "3.0.1"
provwasm-std = { version = "2.8.0", default-features = false, features = ["provenance-name"] }
schemars = "0.8.16"
serde = { version = "1.0.197", default-features = false, features = ["derive"] }
thiserror = { version = "1.0.58" }

[dev-dependencies]
provwasm-mocks = "2.8.0"
```

The CosmWasm 3 build of `provwasm-std` and `provwasm-mocks` is not published yet. Until that release, point both crates at this repository with `path` instead of the version above.

Reset the README and clear out the current JSON schema artifacts.

```bash
echo '# Purchase Smart Contact' > README.md
```

Clear out unnecessary files and directories you don't want hanging around. For example

```bash
rm -rf .circleci Developing.md .git .github .gitpod.Dockerfile .gitpod.yml Importing.md LICENSE NOTICE Publishing.md src/helpers.rs src/integration_tests.rs
```

## Up Next

Proceed to the [Requirements](05-requirements.md) section to define smart contract functionality.
