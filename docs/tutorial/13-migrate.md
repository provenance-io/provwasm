# Provenance Smart Contract Tutorial

In this section we will describe the process up upgrading smart contracts.

## Migrate Contract

Previously, some suggestions were made that would require smart contract code and state updates.
To recap:

1) Change the allowed transfer fee percentage range.
1) Update the transfer fee percentage to a new value in a migrate message.

This was left as an exercise for the reader, so take some time here to update the smart contract
code - remember to update and add more unit tests!

Hints

File: `src/lib.rs`

```rust
#![warn(clippy::all)]
pub mod contract;
pub mod error;
pub mod msg;
pub mod state;

// Make sure migrations are enabled
#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points_with_migration!(contract);
```

File: `src/msg.rs`

```rust
/// Migrate the contract, setting a new fee percentage.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {
    pub new_fee_percent: Decimal,
}
```

File: `examples/schema.rs`

```rust
use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use tutorial::msg::{HandleMsg, InitMsg, MigrateMsg, QueryMsg, QueryResponse};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(HandleMsg), &out_dir);
    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(MigrateMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(QueryResponse), &out_dir);
}
```

File: `src/contract.rs`

```rust
/// Called when migrating a contract instance to a new code ID.
pub fn migrate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MigrateMsg,
) -> Result<MigrateResponse, ContractError> {
    // 1) Ensure the new fee percent is within the updated range.
    // 2) Get mutable ref to the contract state
    // 3) Ensure the message sender is the fee collector
    // 4) Set new fee percent in the state
    Ok(MigrateResponse::default())
}
```

Suggested test stubs

```rust
#[test]
fn valid_migrate() {
    // Test happy-path migrate here
    todo!()
}

#[test]
fn invalid_migrate() {
    // Test migrate error scenarios here
    todo!()
}
```

When complete, use the CLI commands below to migrate the smart contract instance to a new code ID.

### Store

Since WASM can't be updated or changed once deployed, the newly optimized WASM file must be stored
in provenance under a new code ID.

To store the new version, re-run the store command

```bash
provenanced tx wasm store tutorial.wasm \
    --source "https://github.com/provenance-io/provwasm/tree/main/contracts/tutorial-migrate" \
    --builder "cosmwasm/rust-optimizer:0.10.7" \
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

Make sure that the `--source` and `--builder` are updated so third parties can verify this build.

To query for both uploaded code entries

```bash
provenanced q wasm list-code --testnet | jq
```

Should output JSON similar to

```json
[
  {
    "id": 1,
    "creator": "tp10vy2n34wgysds98fmj44jvhm5ugepd4w3pz9s9",
    "data_hash": "B2C5A7BC76D636BEAD7FC4EB51EC77E3F73955B1C8A756AE1FEA5AAFE804912A",
    "source": "https://github.com/provenance-io/provwasm/tree/main/contracts/tutorial",
    "builder": "cosmwasm/rust-optimizer:0.10.7"
  },
  {
    "id": 2,
    "creator": "tp10vy2n34wgysds98fmj44jvhm5ugepd4w3pz9s9",
    "data_hash": "EC22D0FEA3BA5D5368259E34234265EB4A767941212E48CDBD00C5460363C379",
    "source": "https://github.com/provenance-io/provwasm/tree/main/contracts/tutorial-migrate",
    "builder": "cosmwasm/rust-optimizer:0.10.7"
  }
]
```

Copy the new code ID to migrate the current instance.

To query the contract address, query using the original code ID, `1`

```bash
provenanced q wasm list-contract-by-code 1 --testnet | jq
```

Copy the contract `address` field value from the output

```json
[
  {
    "code_id": 1,
    "creator": "tp10vy2n34wgysds98fmj44jvhm5ugepd4w3pz9s9",
    "label": "tutorial-v2",
    "address": "tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"
  }
]
```

### Migrate

Now instead of instantiating a new instance, a separate migrate command can be run

```bash
provenanced tx wasm migrate \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    2 \
    '{"new_fee_percent":"0.05"}' \
    --from feebucket \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 3500vspn \
    --broadcast-mode block \
    --yes \
    --testnet
```

The instance is now migrated to the updated WASM code - with the updated fee range and percent.

To verify, query for contract instances under the new code ID

```bash
provenance q wasm list-contract-by-code 2 | jq
```

Should output JSON simimlar to

```json
[
  {
    "code_id": 2,
    "creator": "tp106xphyzh8fxxdnvp5drharll7724sj9z4rpm6r",
    "label": "tutorial-v2",
    "address": "tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"
  }
]
```

## Up Next

This concludes Part 2 of the tutorial. Proceed to Part 3 to see how to [Integrate](14-integration.md)
Kotlin applications.
