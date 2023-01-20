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
cosmwasm-schema = "1.1.9"
cosmwasm-std = "1.1.9"
cosmwasm-storage = "1.1.9"
cw-storage-plus = "1.0.1"
cw2 = "1.0.1"
provwasm-std = "1.1.2"
schemars = "0.8.10"
serde = { version = "1.0.145", default-features = false, features = ["derive"] }
thiserror = { version = "1.0.31" }

[dev-dependencies]
cw-multi-test = "0.16.2"
provwasm-mocks = "1.1.2"
```

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
