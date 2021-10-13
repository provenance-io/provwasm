# Provenance Smart Contract Tutorial

## Generate Project

CosmWasm has a template repo that can be used as a starting point for development. To generate the
tutorial project, run

```bash
cargo generate --git https://github.com/CosmWasm/cosmwasm-template.git --name tutorial
```

Navigate to the project

```bash
cd ./tutorial
```

Edit Cargo.toml to have the following contract dependencies

```toml
[dependencies]
provwasm-std = { version = "0.16.0"}
cosmwasm-std = { version = "0.16.2" }
cosmwasm-storage = { version = "0.16.2" }
cw-storage-plus = "0.8.0"
cw2 = "0.8.1"
schemars = "0.8.3"
serde = { version = "1.0.127", default-features = false, features = ["derive"] }
thiserror = { version = "1.0.26" }

[dev-dependencies]
provwasm-mocks = { version = "0.16.0" }
cosmwasm-schema = { version = "0.16.2" }
```

Reset the README and clear out the current JSON schema artifacts.

```bash
echo '# Purchase Smart Contact' > README.md
rm -rf schema/*.json
```

Clear out unnecessary files and directories you don't want hanging around. For example

```bash
rm -rf .circleci Developing.md .git .github Importing.md LICENSE NOTICE Publishing.md
```

## Up Next

Proceed to the [Requirements](05-requirements.md) section to define smart contract functionality.
