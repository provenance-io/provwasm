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
```

File: `src/msg.rs`

```rust
/// Migrate the contract, setting a new fee percentage.
#[cw_serde]
pub struct MigrateMsg {
    pub new_fee_percent: Decimal,
}
```

File: `src/contract.rs`

```rust
/// Called when migrating a contract instance to a new code ID.
pub fn migrate(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    msg: MigrateMsg,
) -> Result<Response, ContractError> {
    // 1) Ensure the new fee percent is within the updated range.
    // 2) Get mutable ref to the contract state
    // 3) Set new fee percent in the state
    Ok(Response::default())
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

File: `examples/schema.rs`

```rust
use cosmwasm_schema::write_api;
use tutorial::msg::{ExecuteMsg, InitMsg, MigrateMsg, QueryMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InitMsg,
        migrate: MigrateMsg,
        query: QueryMsg,
    }
}
```

When complete, use the CLI commands below to migrate the smart contract instance to a new code ID.

### Store

Since Wasm can't be updated or changed once deployed, the newly optimized Wasm file must be stored
in provenance under a new code ID.

To store the new version, re-run the store command

```bash
provenanced tx wasm store tutorial.wasm \
    --instantiate-only-address $(provenanced keys show -a feebucket --keyring-backend test --home build/run/provenanced --testnet) \
    --from feebucket \
    --keyring-backend test \
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto \
    --fees 25000nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

To query for both uploaded code entries

```bash
provenanced q wasm list-code --testnet -o json | jq
```

Should output JSON similar to

```json
{
  "code_infos": [
    {
      "code_id": "1",
      "creator": "tp102c9nplcvrxmhevc6wenm99q6dfte3k3z8vscv",
      "data_hash": "F2F2CD9AA2C192A95B86E9429BC15DCD6B8859BE54C8C66274B80347D2443D82",
    },
    {
      "code_id": "2",
      "creator": "tp102c9nplcvrxmhevc6wenm99q6dfte3k3z8vscv",
      "data_hash": "F2F2CD9AA2C192A95B86E9429BC15DCD6B8859BE54C8C66274B80347D2443D82",
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

Copy the new code ID to migrate the current instance.

To query the contract address, query using the original code ID, `1`

```bash
provenanced q wasm list-contract-by-code 1 --testnet -o json | jq
```

Copy the contract address value from the output

```json
{
  "contracts": [
    "tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
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
    --home build/run/provenanced \
    --chain-id testing \
    --gas auto \
    --fees 3500nhash \
    --broadcast-mode block \
    --yes \
    --testnet | jq
```

The instance is now migrated to the updated Wasm code - with the updated fee range and percent.

To verify, query for contract instances under the new code ID

```bash
provenanced q wasm list-contract-by-code 2 -o json | jq
```

Should output JSON similar to

```json
{
  "contracts": [
    "tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"
  ],
  "pagination": {
    "next_key": null,
    "total": "0"
  }
}
```

To verify, query the state of the contract.

```bash
provenanced query wasm contract-state smart \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz '{"query_request":{}}' --testnet -o json | jq
```

Should reflect the updated fee percent.

```json
{
  "data": {
    "purchase_denom": "purchasecoin",
    "merchant_address": "tp1jeqf9m9psa5jvurzpwtdk5m5429fhq48f0u5wq",
    "fee_collection_address": "tp102c9nplcvrxmhevc6wenm99q6dfte3k3z8vscv",
    "fee_percent": "0.05"
  }
}
```
