# Provenance Smart Contract Tutorial

## Development

In this section we will build and test the functionality of the smart contract defined in the
[Requirements](05-requirements.md) section. Replace the contents of the files generated from
template with the the code listed in this section. The best way to learn is to type out the
code. But, it is completely acceptable to copy and paste as well (if time is an issue or, like the
author, the reader is just lazy).

All file locations are referenced from the project root: `.../provwasm/contracts/tutorial`

### Setup

Add a Makefile to combine development build steps into a single `make` command

File: `Makefile`

```Makefile
.PHONY: all
all: fmt build lint test schema

.PHONY: fmt
fmt:
	@cargo fmt

.PHONY: build
build:
	@cargo wasm

.PHONY: lint
lint:
	@cargo clippy

.PHONY: test
test:
	@cargo unit-test

.PHONY: schema
schema:
	@cargo schema
```

NOTE: A few of these cargo commands are aliases. The full commands can be seen in the
`.cargo/config` file.

```toml
[alias]
wasm = "build --release --target wasm32-unknown-unknown"
unit-test = "test --lib --features backtraces"
schema = "run --example schema"
```

### Library

File: `src/lib.rs`

The main project file. Adds smart contract WASM entry points and ensures all project linter
warnings are checked.

```rust
#![warn(clippy::all)]
pub mod contract;
pub mod error;
pub mod msg;
pub mod state;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
```

### Errors

File: `src/error.rs`

Adds customizable errors to the smart contract.

```rust
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
}
```

### State

File: `src/state.rs`

Defines a singleton (one key, one value) configuration state for the smart contract.

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Decimal, HumanAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

/// Fields that comprise the smart contract state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    // The contract name
    pub contract_name: String,
    // The required purchase denomination
    pub purchase_denom: String,
    // The merchant account
    pub merchant_address: HumanAddr,
    // The fee collection account
    pub fee_collection_address: HumanAddr,
    // The percentage to collect on transfers
    pub fee_percent: Decimal,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONFIG_KEY)
}
```

### Messages

File: `src/msg.rs`

Define message types for the smart contract.

```rust
use cosmwasm_std::{Decimal, HumanAddr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::State;

/// A message sent to initialize the contract state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub contract_name: String,
    pub purchase_denom: String,
    pub merchant_address: HumanAddr,
    pub fee_percent: Decimal,
}

/// A message sent to transfer funds and collect fees for a purchase.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Purchase { id: String },
}

/// A message sent to query contract config state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryRequest {},
}

/// A type alias for contract state.
pub type QueryResponse = State;
```

### JSON Schema

File: `examples/schema.rs`

Ensure a JSON schema is generated for the smart contract types.

```rust
use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use tutorial::msg::{HandleMsg, InitMsg, QueryMsg, QueryResponse};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(HandleMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(QueryResponse), &out_dir);
}
```

### Message Handlers

File: `src/contract.rs`

#### Imports

The following imports are required for the init, query and handle functions.

```rust
use cosmwasm_std::{
    attr, coin, to_binary, BankMsg, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env, HandleResponse,
    InitResponse, MessageInfo, StdError, StdResult,
};
use provwasm_std::{bind_name, ProvenanceMsg};
use std::ops::Mul;

use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};
```

#### Init

Handler code for contract instantiation.

```rust
/// Initialize the contract
pub fn init(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<InitResponse<ProvenanceMsg>, StdError> {
    // Ensure no funds were sent with the message
    if !info.sent_funds.is_empty() {
        let errm = "purchase funds are not allowed to be sent during init";
        return Err(StdError::generic_err(errm));
    }

    // Ensure there are limits on fees.
    if msg.fee_percent.is_zero() || msg.fee_percent > Decimal::percent(25) {
        return Err(StdError::generic_err(
            "fee percent must be > 0.0 and <= 0.25",
        ));
    }

    // Ensure the merchant address is not also the fee collection address
    if msg.merchant_address == info.sender {
        return Err(StdError::generic_err(
            "merchant address can't be the fee collection address",
        ));
    }

    // Create and save contract config state. The fee collection address represents the network
    // (ie they get paid fees), thus they must be the message sender.
    let state = State {
        contract_name: msg.contract_name.clone(),
        purchase_denom: msg.purchase_denom,
        merchant_address: msg.merchant_address,
        fee_collection_address: info.sender,
        fee_percent: msg.fee_percent,
    };
    config(deps.storage).save(&state)?;

    // Issue a message that will bind a restricted name to the contract address and emit an event.
    let bind_name_msg = bind_name(msg.contract_name, env.contract.address);
    Ok(InitResponse {
        messages: vec![bind_name_msg],
        attributes: vec![attr("tutorial-v2", ""), attr("action", "tutorial.init")],
    })
}
```

#### Query

Query code for accessing contract state.

```rust
/// Query contract state.
pub fn query(
    deps: Deps,
    _env: Env, // NOTE: A '_' prefix indicates a variable is unused (supress linter warnings)
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryRequest {} => {
            let state = config_read(deps.storage).load()?;
            let json = to_binary(&state)?;
            Ok(json)
        }
    }
}
```

#### Handle

```rust
/// Handle purchase messages.
pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse<BankMsg>, ContractError> {
    match msg {
        HandleMsg::Purchase { id } => try_purchase(deps, env, info, id),
    }
}

// Calculates transfers and fees, then dispatches messages to the bank module.
fn try_purchase(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<HandleResponse<BankMsg>, ContractError> {
    // Ensure funds were sent with the message
    if info.sent_funds.is_empty() {
        let errm = "no purchase funds sent";
        return Err(ContractError::Std(StdError::generic_err(errm)));
    }

    // Load state
    let state = config_read(deps.storage).load()?;
    let fee_pct = state.fee_percent;

    // Ensure the funds have the required amount and denomination
    for funds in info.sent_funds.iter() {
        if funds.amount.is_zero() || funds.denom != state.purchase_denom {
            let errm = format!("invalid purchase funds: {}{}", funds.amount, funds.denom);
            return Err(ContractError::Std(StdError::generic_err(errm)));
        }
    }

    // Calculate amounts and create bank transfers to the merchant account
    let transfers = CosmosMsg::Bank(BankMsg::Send {
        from_address: env.contract.address.clone(),
        to_address: state.merchant_address,
        amount: info
            .sent_funds
            .iter()
            .map(|sent| {
                let fees = sent.amount.mul(fee_pct).u128();
                coin(sent.amount.u128() - fees, sent.denom.clone())
            })
            .collect(),
    });

    // Calculate fees and create bank transfers to the fee collection account
    let fees = CosmosMsg::Bank(BankMsg::Send {
        from_address: env.contract.address,
        to_address: state.fee_collection_address,
        amount: info
            .sent_funds
            .iter()
            .map(|sent| coin(sent.amount.mul(fee_pct).u128(), sent.denom.clone()))
            .collect(),
    });

    // Return a response that will dispatch the transfers to the bank module and emit events.
    Ok(HandleResponse {
        messages: vec![transfers, fees],
        attributes: vec![
            attr("tutorial-v2", ""),
            attr("action", "tutorial.purchase"),
            attr("purchase_id", id),
            attr("purchase_time", env.block.time), // Use BFT time as event timestamp
        ],
        data: None,
    })
}
```

### Unit Tests

File: `src/contract.rs`

Add an inner module with imports for contract unit tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::QueryResponse;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{from_binary, HumanAddr};
    use provwasm_mocks::mock_dependencies;

    // tests here...
}
```

Add a unit test that establishes a valid init call works

```rust
#[test]
fn valid_init() {
    // Create mocks
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("feebucket", &[]);

    // Create a valid init message
    let msg = InitMsg {
        contract_name: "tutorial.sc.pb".into(),
        purchase_denom: "fpcoin".into(),
        merchant_address: HumanAddr::from("merchant"),
        fee_percent: Decimal::percent(10),
    };

    // Ensure a message was created to bind the name to the contract address.
    let res = init(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(res.messages.len(), 1);
}
```

Add a unit test to ensure an invalid merchant address sent to `init` causes an error

```rust
#[test]
fn invalid_merchant_init() {
    // Create mocks
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("merchant", &[]); // error: merchant can't be fee collector

    // Create an invalid init message
    let msg = InitMsg {
        contract_name: "tutorial.sc.pb".into(),
        purchase_denom: "fpcoin".into(),
        merchant_address: HumanAddr::from("merchant"),
        fee_percent: Decimal::percent(10),
    };

    // Ensure the expected error was returned.
    let err = init(deps.as_mut(), env, info, msg).unwrap_err();
    match err {
        StdError::GenericErr { msg, .. } => {
            assert_eq!(msg, "merchant address can't be the fee collection address")
        }
        _ => panic!("unexpected init error"),
    }
}
```

Add a unit test to ensure an invalid fee percentage sent to `init` causes an error

```rust
#[test]
fn invalid_fee_percent_init() {
    // Create mocks
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("feebucket", &[]);

    // Create an invalid init message
    let msg = InitMsg {
        contract_name: "tutorial.sc.pb".into(),
        purchase_denom: "fpcoin".into(),
        merchant_address: HumanAddr::from("merchant"),
        fee_percent: Decimal::percent(37), // error: > 25%
    };

    // Ensure the expected error was returned.
    let err = init(deps.as_mut(), env, info, msg).unwrap_err();
    match err {
        StdError::GenericErr { msg, .. } => {
            assert_eq!(msg, "fee percent must be > 0.0 and <= 0.25")
        }
        _ => panic!("unexpected init error"),
    }
}
```

Add a test to ensure state set during init can be queried.

```rust
#[test]
fn query_test() {
    // Create mocks
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("feebucket", &[]);

    // Create a valid init message to store contract state.
    let msg = InitMsg {
        contract_name: "tutorial.sc.pb".into(),
        purchase_denom: "fpcoin".into(),
        merchant_address: HumanAddr::from("merchant"),
        fee_percent: Decimal::percent(10),
    };
    let _ = init(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

    // Call the smart contract query function to get stored state.
    let msg = QueryMsg::QueryRequest {};
    let bin = query(deps.as_ref(), mock_env(), msg).unwrap();
    let resp: QueryResponse = from_binary(&bin).unwrap();

    // Ensure the expected init fields were properly stored.
    assert_eq!(resp.contract_name, "tutorial.sc.pb");
    assert_eq!(resp.merchant_address, HumanAddr::from("merchant"));
    assert_eq!(resp.purchase_denom, "fpcoin");
    assert_eq!(resp.fee_collection_address, HumanAddr::from("feebucket"));
    assert_eq!(resp.fee_percent, Decimal::percent(10));
}
```

Add a unit test that establishes a valid purchase handle call works

```rust
#[test]
fn handle_valid_purchase() {
    // Init contract state
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("feebucket", &[]);
    let msg = InitMsg {
        contract_name: "tutorial.sc.pb".into(),
        purchase_denom: "fpcoin".into(),
        merchant_address: HumanAddr::from("merchant"),
        fee_percent: Decimal::percent(10),
    };
    let _ = init(deps.as_mut(), env, info, msg).unwrap();

    // Send a valid purchase message of 100fpcoin
    let msg = HandleMsg::Purchase {
        id: "a7918172-ac09-43f6-bc4b-7ac2fbad17e9".into(),
    };
    let info = mock_info("consumer", &[coin(100, "fpcoin")]);
    let res = handle(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Ensure we have the merchant transfer and fee collection bank messages
    assert_eq!(res.messages.len(), 2);

    // Ensure we got the proper bank transfer values.
    // 10% fees on 100 fpcoin => 90 fpcoin for the merchant and 10 fpcoin for the fee bucket.
    let expected_transfer = coin(90, "fpcoin");
    let expected_fees = coin(10, "fpcoin");
    res.messages.into_iter().for_each(|msg| match msg {
        CosmosMsg::Bank(BankMsg::Send {
            amount, to_address, ..
        }) => {
            assert_eq!(amount.len(), 1);
            if to_address == HumanAddr::from("merchant") {
                assert_eq!(amount[0], expected_transfer)
            } else if to_address == HumanAddr::from("feebucket") {
                assert_eq!(amount[0], expected_fees)
            } else {
                panic!("unexpected to_address in bank message")
            }
        }
        _ => panic!("unexpected message type"),
    });

    // Ensure we got the purchase ID event attribute value
    let expected_purchase_id = "a7918172-ac09-43f6-bc4b-7ac2fbad17e9";
    res.attributes.into_iter().for_each(|atr| {
        if atr.key == "purchase_id" {
            assert_eq!(atr.value, expected_purchase_id)
        }
    })
}
```

Add a unit test to ensure invalid purchase messages cause errors

```rust
#[test]
fn handle_invalid_funds() {
    // Init contract state
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("feebucket", &[]);
    let msg = InitMsg {
        contract_name: "tutorial.sc.pb".into(),
        purchase_denom: "fpcoin".into(),
        merchant_address: HumanAddr::from("merchant"),
        fee_percent: Decimal::percent(10),
    };
    let _ = init(deps.as_mut(), env, info, msg).unwrap();

    // Test purchase message
    let msg = HandleMsg::Purchase {
        id: "a7918172-ac09-43f6-bc4b-7ac2fbad17e9".into(),
    };

    // Don't send any funds
    let info = mock_info("consumer", &[]);
    let err = handle(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
    match err {
        ContractError::Std(StdError::GenericErr { msg, .. }) => {
            assert_eq!(msg, "no purchase funds sent")
        }
        _ => panic!("unexpected handle error"),
    }

    // Send zero amount for a valid denom
    let info = mock_info("consumer", &[coin(0, "fpcoin")]);
    let err = handle(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
    match err {
        ContractError::Std(StdError::GenericErr { msg, .. }) => {
            assert_eq!(msg, "invalid purchase funds: 0fpcoin")
        }
        _ => panic!("unexpected handle error"),
    }

    // Send invalid denom
    let info = mock_info("consumer", &[coin(100, "fakecoin")]);
    let err = handle(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    match err {
        ContractError::Std(StdError::GenericErr { msg, .. }) => {
            assert_eq!(msg, "invalid purchase funds: 100fakecoin")
        }
        _ => panic!("unexpected handle error"),
    }
}
```

## Build

Build and edit the smart contract source code until all compiler errors are resolved.

```bash
make
```

## Up Next

Proceed to the next [section](07-optimize.md) to optimize the smart contract WASM for deployment.
