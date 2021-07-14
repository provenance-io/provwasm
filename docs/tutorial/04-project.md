# Provenance Smart Contract Tutorial

## Generate Project

CosmWasm has a template repo that can be used as a starting point for development. To generate the
tutorial project, run

```bash
cargo generate --git https://github.com/CosmWasm/cosmwasm-template.git --branch 0.14 --name tutorial
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
provwasm-std = { version = "0.14.2"}
cosmwasm-std = { version = "0.14.1" }
cosmwasm-storage = { version = "0.14.1" }
schemars = "0.8.1"
serde = { version = "1.0.103", default-features = false, features = ["derive"] }
thiserror = { version = "1.0.20" }

[dev-dependencies]
provwasm-mocks = { version = "0.14.2" }
cosmwasm-schema = { version = "0.14.1" }
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
