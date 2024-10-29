# Provenance Smart Contract Tutorial

## Development

In this section we will build and test the functionality of the smart contract defined in the
[Requirements](05-requirements.md) section. Replace the contents of the files generated from template with the the code
listed in this section. The best way to learn is to type out the code. But, it is completely acceptable to copy and
paste as well.

### Setup

Add a Makefile to combine development build steps into a single `make` command

File: `Makefile`

```Makefile
.PHONY: all
all: fmt build test lint schema optimize

.PHONY: pre-optimize
pre-optimize: fmt build test lint schema

UNAME_M := $(shell uname -m)

.PHONY: fmt
fmt:
	@cargo fmt --all -- --check

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@RUST_BACKTRACE=1 cargo unit-test

.PHONY: lint
lint:
	@cargo clippy

.PHONY: schema
schema:
	@cargo schema

.PHONY: clean
clean:
	@cargo clean
	@cargo clean --target-dir artifacts
```

NOTE: A few of these cargo commands are aliases. The full commands can be seen in the
`.cargo/config` file.

```toml
[alias]
wasm = "build --release --target wasm32-unknown-unknown"
unit-test = "test --lib"
schema = "run --example schema"
```

### Library

File: `src/lib.rs`

The main project file. Adds smart contract Wasm entry points and ensures all project linter
warnings are checked.

```rust
pub mod contract;  
mod error;  
pub mod msg;  
pub mod state;  
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
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CONFIG: Item<State> = Item::new("config");

/// Fields that comprise the smart contract state
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    // The required purchase denomination
    pub purchase_denom: String,
    // The merchant account
    pub merchant_address: Addr,
    // The fee collection account
    pub fee_collection_address: Addr,
    // The percentage to collect on transfers
    pub fee_percent: Decimal,
}
```

### Messages

File: `src/msg.rs`

Define message types for the smart contract.

```rust
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;

/// A message sent to initialize the contract state.
#[cw_serde]
pub struct InitMsg {
    pub contract_name: String,
    pub purchase_denom: String,
    pub merchant_address: String,
    pub fee_percent: Decimal,
}

/// A message sent to transfer funds and collect fees for a purchase.
#[cw_serde]
pub enum ExecuteMsg {
    Purchase { id: String },
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}

/// A message sent to query contract config state.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::State)]
    QueryRequest {},
}
```

### Message Handlers

File: `src/contract.rs`

#### Imports

The following imports are required for the init, query and handle functions.

```rust
use cosmwasm_std::{
    coin, entry_point, to_json_binary, BankMsg, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult,
};
use provwasm_std::types::provenance::name::v1::{MsgBindNameRequest, NameRecord};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InitMsg, MigrateMsg, QueryMsg};
use crate::state::{State, CONFIG};
```

#### Instantiate

Handler code for contract instantiation.

```rust
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, StdError> {
    // Ensure no funds were sent with the message
    if !info.funds.is_empty() {
        let err = "purchase funds are not allowed to be sent during init";
        return Err(StdError::generic_err(err));
    }

    // Ensure there are limits on fees.
    if msg.fee_percent.is_zero() || msg.fee_percent > Decimal::percent(25) {
        return Err(StdError::generic_err(
            "fee percent must be > 0.0 and <= 0.25",
        ));
    }

    // Ensure the merchant address is not also the fee collection address
    if msg.merchant_address.eq(&info.sender.to_string()) {
        return Err(StdError::generic_err(
            "merchant address can't be the fee collection address",
        ));
    }

    // Create and save contract config state. The fee collection address represents the network
    // (ie they get paid fees), thus they must be the message sender.
    let merchant_address = deps.api.addr_validate(&msg.merchant_address)?;
    CONFIG.save(
        deps.storage,
        &State {
            purchase_denom: msg.purchase_denom,
            merchant_address,
            fee_collection_address: info.sender,
            fee_percent: msg.fee_percent,
        },
    )?;

    // Create a message that will bind a restricted name to the contract address.
    let split: Vec<&str> = msg.contract_name.splitn(2, '.').collect();
    let record = split.first();
    let parent = split.last();

    match (parent, record) {
        (Some(parent), Some(record)) => {
            // Create a bind name message
            let bind_name_msg = MsgBindNameRequest {
                parent: Some(NameRecord {
                    name: parent.to_string(),
                    address: env.contract.address.to_string(),
                    restricted: true,
                }),
                record: Some(NameRecord {
                    name: record.to_string(),
                    address: env.contract.address.to_string(),
                    restricted: true,
                }),
            };

            // Dispatch bind name message and add event attributes.
            let res = Response::new()
                .add_message(bind_name_msg)
                .add_attribute("action", "init");
            Ok(res)
        }
        (_, _) => Err(StdError::generic_err("Invalid contract name")),
    }
}
```

#### Query

Query code for accessing contract state.

```rust
#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env, // NOTE: A '_' prefix indicates a variable is unused (suppress linter warnings)
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryRequest {} => {
            let state = CONFIG.load(deps.storage)?;
            let json = to_json_binary(&state)?;
            Ok(json)
        }
    }
}
```

#### Execute

```rust
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // BankMsg
    match msg {
        ExecuteMsg::Purchase { id } => try_purchase(deps, env, info, id),
    }
}
  
// Calculates transfers and fees, then dispatches messages to the bank module.
fn try_purchase(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    // Ensure funds were sent with the message
    if info.funds.is_empty() {
        let err = "no purchase funds sent";
        return Err(ContractError::Std(StdError::generic_err(err)));
    }

    // Load state
    let state = CONFIG.load(deps.storage)?;
    let fee_pct = state.fee_percent;

    // Ensure the funds have the required amount and denomination
    for funds in info.funds.iter() {
        if funds.amount.is_zero() || funds.denom != state.purchase_denom {
            let err = format!("invalid purchase funds: {}{}", funds.amount, funds.denom);
            return Err(ContractError::Std(StdError::generic_err(err)));
        }
    }

    // Calculate amounts and create bank transfers to the merchant account
    let transfers = CosmosMsg::Bank(BankMsg::Send {
        to_address: state.merchant_address.to_string(),
        amount: info
            .funds
            .iter()
            .map(|sent| {
                let fees = sent.amount.mul_floor(fee_pct).u128();
                coin(sent.amount.u128() - fees, sent.denom.clone())
            })
            .collect(),
    });

    // Calculate fees and create bank transfers to the fee collection account
    let fees = CosmosMsg::Bank(BankMsg::Send {
        to_address: state.fee_collection_address.to_string(),
        amount: info
            .funds
            .iter()
            .map(|sent| coin(sent.amount.mul_floor(fee_pct).u128(), sent.denom.clone()))
            .collect(),
    });

    // Return a response that will dispatch the transfers to the bank module and emit events.
    Ok(Response::new()
        .add_message(transfers)
        .add_message(fees)
        .add_attribute("action", "purchase")
        .add_attribute("purchase_id", id)
        .add_attribute("purchase_time", env.block.time.to_string()))
}
```

#### Migrate

```rust
/// Called when migrating a contract instance to a new code id.  
#[entry_point]  
pub fn migrate(  
    _deps: DepsMut<ProvenanceQuery>,  
    _env: Env,  
    _msg: MigrateMsg,  
) -> Result<Response, ContractError> {  
    Ok(Response::default())  
}
```

### Unit Tests

File: `src/contract.rs`

Add an inner module with imports for contract unit tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{message_info, mock_env};
    use cosmwasm_std::{from_json, Addr, AnyMsg, Binary, CosmosMsg};
    use provwasm_mocks::mock_provenance_dependencies;
    use provwasm_std::types::provenance::name::v1::{
        QueryResolveRequest, QueryResolveResponse, QueryReverseLookupRequest,
        QueryReverseLookupResponse,
    };

    #[test]
    fn init_test() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);

        // Give the contract a name
        let msg = InitMsg {
            name: "contract.pb".into(),
        };

        let contract_address = env.contract.address.to_string();

        // Ensure a message was created to bind the name to the contract address.
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        match &res.messages[0].msg {
            CosmosMsg::Any(AnyMsg { type_url, value }) => {
                let expected: Binary = MsgBindNameRequest {
                    parent: Some(NameRecord {
                        name: "pb".to_string(),
                        address: contract_address.clone(),
                        restricted: true,
                    }),
                    record: Some(NameRecord {
                        name: "contract".to_string(),
                        address: contract_address,
                        restricted: true,
                    }),
                }
                .into();

                assert_eq!(type_url, "/provenance.name.v1.MsgBindNameRequest");
                assert_eq!(value, &expected)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn bind_name_success() {
        // Init state
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = instantiate(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Bind a name
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = ExecuteMsg::BindPrefix {
            prefix: "test".into(),
        };

        let contract_address = env.contract.address.to_string();

        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        // Assert the correct message was created
        match &res.messages[0].msg {
            CosmosMsg::Any(AnyMsg { type_url, value }) => {
                let expected: Binary = MsgBindNameRequest {
                    parent: Some(NameRecord {
                        name: "contract.pb".to_string(),
                        address: contract_address.clone(),
                        restricted: true,
                    }),
                    record: Some(NameRecord {
                        name: "test".to_string(),
                        address: contract_address,
                        restricted: true,
                    }),
                }
                .into();

                assert_eq!(type_url, "/provenance.name.v1.MsgBindNameRequest");
                assert_eq!(value, &expected)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn unbind_name_success() {
        // Init state
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = instantiate(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Bind a name
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = ExecuteMsg::UnbindPrefix {
            prefix: "test".into(),
        };

        let contract_address = env.contract.address.to_string();

        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        // Assert the correct message was created
        assert_eq!(1, res.messages.len());
        match &res.messages[0].msg {
            CosmosMsg::Any(AnyMsg { type_url, value }) => {
                let expected: Binary = MsgDeleteNameRequest {
                    record: Some(NameRecord {
                        name: "test.contract.pb".to_string(),
                        address: contract_address,
                        restricted: true,
                    }),
                }
                .into();

                assert_eq!(type_url, "/provenance.name.v1.MsgDeleteNameRequest");
                assert_eq!(value, &expected)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn bind_name_unauthorized() {
        // Init state
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = instantiate(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Try to bind a name with some other sender address
        let env = mock_env();
        let info = message_info(&Addr::unchecked("other"), &[]); // error: not 'sender'
        let msg = ExecuteMsg::BindPrefix {
            prefix: "test".into(),
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();

        // Assert an unauthorized error was returned
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn unbind_name_unauthorized() {
        // Init state
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = instantiate(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Try to bind a name with some other sender address
        let env = mock_env();
        let info = message_info(&Addr::unchecked("other"), &[]); // error: not 'sender'
        let msg = ExecuteMsg::UnbindPrefix {
            prefix: "test".into(),
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();

        // Assert an unauthorized error was returned
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn query_resolve() {
        // Create provenance mock deps with a single bound name.

        let mut deps = mock_provenance_dependencies();

        let mock_response = QueryResolveResponse {
            address: "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync".to_string(),
            restricted: false,
        };

        QueryResolveRequest::mock_response(&mut deps.querier, mock_response);

        // Call the smart contract query function to resolve the address for our test name.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Resolve {
                name: "a.pb".into(),
            },
        )
        .unwrap();

        // Ensure that we got the expected address.
        let rep: String = from_json(bin).unwrap();
        assert_eq!(rep, "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync")
    }

    #[test]
    fn query_lookup() {
        // Create provenance mock deps with two bound names.
        let mut deps = mock_provenance_dependencies();

        let mock_response = QueryReverseLookupResponse {
            name: vec!["b.pb".to_string(), "a.pb".to_string()],
            pagination: None,
        };

        QueryReverseLookupRequest::mock_response(&mut deps.querier, mock_response.clone());

        // Call the smart contract query function to lookup names bound to an address.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Lookup {
                address: deps.api.addr_make("address").into(),
            },
        )
        .unwrap();

        // Ensure that we got the expected number of records.
        let rep: LookupResponse = from_json(bin).unwrap();
        assert_eq!(
            rep,
            LookupResponse {
                name: vec!["b.pb".to_string(), "a.pb".to_string()]
            }
        );
    }

    #[test]
    fn query_lookup_empty() {
        // Create provenance mock deps with a bound name.
        let mut deps = mock_provenance_dependencies();
        let mock_response = QueryReverseLookupResponse {
            name: vec![],
            pagination: None,
        };

        QueryReverseLookupRequest::mock_response(&mut deps.querier, mock_response.clone());

        // Call the smart contract query function to lookup names bound to an address.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Lookup {
                address: deps.api.addr_make("address2").into(),
            },
        )
        .unwrap();

        // Ensure that we got zero records.
        let rep: LookupResponse = from_json(bin).unwrap();
        assert_eq!(rep, LookupResponse { name: vec![] });
    }
}
```

### JSON Schema

File: `src/bin/schema.rs`

Ensure a JSON schema is generated for the smart contract types.

```rust
use cosmwasm_schema::write_api;
use name::msg::{ExecuteMsg, InitMsg, QueryMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InitMsg,
        query: QueryMsg,
    }
}
```

## Code Format

Before building make sure that everything is formatted correctly using:

```bash
cargo fmt
```

## Build

Build and edit the smart contract source code until all compiler errors are resolved.

```bash
make
```

## Up Next

Proceed to the next [section](07-optimize.md) to optimize the smart contract Wasm for deployment.
