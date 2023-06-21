# Provenance Smart Contract Tutorial

## Development

In this section we will build and test the functionality of the smart contract defined in the
[Requirements](05-requirements.md) section. Replace the contents of the files generated from template with the the code listed in this section. The best way to learn is to type out the code. But, it is completely acceptable to copy and paste as well.

### Setup

Add a Makefile to combine development build steps into a single `make` command

File: `Makefile`

```Makefile
.PHONY: all
all: fmt build test lint schema optimize

.PHONY: fmt
fmt:
	@cargo fmt --all -- --check

.PHONY: build
build:
	@cargo wasm

.PHONY: test
test:
	@RUST_BACKTRACE=1 cargo unit-test

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: schema
schema:
	@cargo schema
```

NOTE: A few of these cargo commands are aliases. The full commands can be seen in the
`.cargo/config` file.

```toml
[alias]
wasm = "build --release --target wasm32-unknown-unknown"  
unit-test = "test --lib"
schema = "run --bin schema"
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
  
pub use crate::error::ContractError;
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
  
use cosmwasm_std::{Addr, Decimal};  
use cw_storage_plus::Item;  
  
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]  
pub struct Config {  
    // The required purchase denomination  
    pub purchase_denom: String,  
    // The merchant account  
    pub merchant_address: Addr,  
    // The fee collection account  
    pub fee_collection_address: Addr,  
    // The percentage to collect on transfers  
    pub fee_percent: Decimal,  
}  
  
pub const CONFIG: Item<Config> = Item::new("config");
```

### Messages

File: `src/msg.rs`

Define message types for the smart contract.

```rust
use crate::state::Config;  
use cosmwasm_schema::{cw_serde, QueryResponses};  
use cosmwasm_std::Decimal;  
  
/// A message sent to initialize the contract state.  
#[cw_serde]  
pub struct InstantiateMsg {  
    pub contract_name: String,  
    pub purchase_denom: String,  
    pub merchant_address: Addr,  
    pub fee_percent: Decimal,  
}  
  
/// A message sent to transfer funds and collect fees for a purchase.  
#[cw_serde]  
pub enum ExecuteMsg {  
    Purchase { id: String },  
}  
  
/// A message sent to migrate the contract to a new code id.  
#[cw_serde]  
pub enum MigrateMsg {}  
  
/// A message sent to query contract config state.  
#[cw_serde]  
#[derive(QueryResponses)]  
pub enum QueryMsg {  
    #[returns(ConfigResponse)]  
    QueryRequest {},  
}  
  
/// A response .  
#[cw_serde]  
pub struct ConfigResponse {  
    pub config: Config,  
}
```

### Message Handlers

File: `src/contract.rs`

#### Imports

The following imports are required for the init, query and handle functions.

```rust
use cosmwasm_std::{  
    coin, entry_point, to_binary, BankMsg, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env,  
    MessageInfo, Response, StdError, StdResult,  
};  
use cw2::set_contract_version;  
use provwasm_std::{bind_name, NameBinding, ProvenanceMsg, ProvenanceQuery};  
use std::ops::Mul;  
  
use crate::error::ContractError;  
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};  
use crate::state::{Config, CONFIG};
```

#### Instantiate

Handler code for contract instantiation.

```rust  
// version info for migration info  
const CONTRACT_NAME: &str = "crates.io:tutorial";  
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");  
  
#[entry_point]  
pub fn instantiate(  
    deps: DepsMut<ProvenanceQuery>,  
    env: Env,  
    info: MessageInfo,  
    msg: InstantiateMsg,  
) -> Result<Response<ProvenanceMsg>, ContractError> {  
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;  
  
    // Ensure no funds were sent with the message  
    if !info.funds.is_empty() {  
        let err = "purchase funds are not allowed to be sent during init";  
        return Err(ContractError::Std(StdError::generic_err(err)));  
    }  
  
    // Ensure there are limits on fees.  
    if msg.fee_percent.is_zero() || msg.fee_percent > Decimal::percent(25) {  
        return Err(ContractError::Std(StdError::generic_err(  
            "fee percent must be > 0.0 and <= 0.25",  
        )));  
    }  
  
    // Ensure the merchant address is not also the fee collection address  
    if msg.merchant_address == info.sender {  
        return Err(ContractError::Std(StdError::generic_err(  
            "merchant address can't be the fee collection address",  
        )));  
    }  
  
    // Create and save contract config state. The fee collection address represents the network  
    // (ie they get paid fees), thus they must be the message sender.    let merchant_address = deps.api.addr_validate(&msg.merchant_address)?;  
    CONFIG.save(  
        deps.storage,  
        &Config {  
            purchase_denom: msg.purchase_denom,  
            merchant_address,  
            fee_collection_address: info.sender,  
            fee_percent: msg.fee_percent,  
        },  
    )?;  
  
    // Create a message that will bind a restricted name to the contract address.  
    let msg = bind_name(  
        &msg.contract_name,  
        env.contract.address,  
        NameBinding::Restricted,  
    )?;  
  
    Ok(Response::new()  
        .add_message(msg)  
        .add_attribute("action", "init"))  
}
```

#### Query

Query code for accessing contract state.

```rust
#[entry_point]  
pub fn query(  
    deps: Deps<ProvenanceQuery>,  
    _env: Env, // NOTE: A '_' prefix indicates a variable is unused (suppress linter warnings)  
    msg: QueryMsg,  
) -> StdResult<Binary> {  
    match msg {  
        QueryMsg::QueryRequest {} => {  
            let state = CONFIG.load(deps.storage)?;  
            let json = to_binary(&state)?;  
            Ok(json)  
        }  
    }  
}
```

#### Execute

```rust
#[entry_point]  
pub fn execute(  
    deps: DepsMut<ProvenanceQuery>,  
    env: Env,  
    info: MessageInfo,  
    msg: ExecuteMsg,  
) -> Result<Response<ProvenanceMsg>, ContractError> {  
    match msg {  
        ExecuteMsg::Purchase { id } => try_purchase(deps, env, info, id),  
    }  
}  
  
// Calculates transfers and fees, then dispatches messages to the bank module.  
fn try_purchase(  
    deps: DepsMut<ProvenanceQuery>,  
    env: Env,  
    info: MessageInfo,  
    id: String,  
) -> Result<Response<ProvenanceMsg>, ContractError> {  
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
                let fees = sent.amount.mul(fee_pct).u128();  
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
            .map(|sent| coin(sent.amount.mul(fee_pct).u128(), sent.denom.clone()))  
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
    use crate::state::Config;  
    use cosmwasm_std::testing::{mock_env, mock_info};  
    use cosmwasm_std::{from_binary, Addr};  
    use provwasm_mocks::mock_dependencies;  
    use provwasm_std::{NameMsgParams, ProvenanceMsgParams};  
  
    #[test]  
    fn valid_init() {  
        // Create mocks  
        let mut deps = mock_dependencies(&[]);  
  
        // Create valid config state  
        let res = instantiate(  
            deps.as_mut(),  
            mock_env(),  
            mock_info("feebucket", &[]),  
            InstantiateMsg {  
                contract_name: "tutorial.sc.pb".into(),  
                purchase_denom: "purchasecoin".into(),  
                merchant_address: Addr::unchecked("merchant"),  
                fee_percent: Decimal::percent(10),  
            },  
        )  
        .unwrap();  
  
        // Ensure a message was created to bind the name to the contract address.  
        assert_eq!(res.messages.len(), 1);  
        match &res.messages[0].msg {  
            CosmosMsg::Custom(msg) => match &msg.params {  
                ProvenanceMsgParams::Name(p) => match &p {  
                    NameMsgParams::BindName { name, .. } => assert_eq!(name, "tutorial.sc.pb"),  
                    _ => panic!("unexpected name params"),  
                },  
                _ => panic!("unexpected provenance params"),  
            },  
            _ => panic!("unexpected cosmos message"),  
        }  
    }  
  
    #[test]  
    fn invalid_merchant_init() {  
        // Create mocks  
        let mut deps = mock_dependencies(&[]);  
  
        // Create an invalid init message  
        let err = instantiate(  
            deps.as_mut(),  
            mock_env(),  
            mock_info("merchant", &[]),  
            InstantiateMsg {  
                contract_name: "tutorial.sc.pb".into(),  
                purchase_denom: "purchasecoin".into(),  
                merchant_address: Addr::unchecked("merchant"),  
                fee_percent: Decimal::percent(10),  
            },  
        )  
        .unwrap_err();  
  
        // Ensure the expected error was returned.  
        match err {  
            ContractError::Std(StdError::GenericErr { msg, .. }) => {  
                assert_eq!(msg, "merchant address can't be the fee collection address")  
            }  
            _ => panic!("unexpected init error"),  
        }  
    }  
  
    #[test]  
    fn invalid_fee_percent_init() {  
        // Create mocks  
        let mut deps = mock_dependencies(&[]);  
  
        // Create an invalid init message.  
        let err = instantiate(  
            deps.as_mut(),  
            mock_env(),  
            mock_info("feebucket", &[]),  
            InstantiateMsg {  
                contract_name: "tutorial.sc.pb".into(),  
                purchase_denom: "purchasecoin".into(),  
                merchant_address: Addr::unchecked("merchant"),  
                fee_percent: Decimal::percent(37), // error: > 25%  
            },  
        )  
        .unwrap_err();  
  
        // Ensure the expected error was returned  
        match err {  
            ContractError::Std(StdError::GenericErr { msg, .. }) => {  
                assert_eq!(msg, "fee percent must be > 0.0 and <= 0.25")  
            }  
            _ => panic!("unexpected init error"),  
        }  
    }  
  
    #[test]  
    fn query_test() {  
        // Create mocks  
        let mut deps = mock_dependencies(&[]);  
  
        // Create config state  
        instantiate(  
            deps.as_mut(),  
            mock_env(),  
            mock_info("feebucket", &[]),  
            InstantiateMsg {  
                contract_name: "tutorial.sc.pb".into(),  
                purchase_denom: "purchasecoin".into(),  
                merchant_address: Addr::unchecked("merchant"),  
                fee_percent: Decimal::percent(10),  
            },  
        )  
        .unwrap(); // Panics on error  
  
        // Call the smart contract query function to get stored state.        
        let bin = query(deps.as_ref(), mock_env(), QueryMsg::QueryRequest {}).unwrap();  
        let resp: Config = from_binary(&bin).unwrap();  
  
        // Ensure the expected init fields were properly stored.  
        assert_eq!(resp.merchant_address, Addr::unchecked("merchant"));  
        assert_eq!(resp.purchase_denom, "purchasecoin");  
        assert_eq!(resp.fee_collection_address, Addr::unchecked("feebucket"));  
        assert_eq!(resp.fee_percent, Decimal::percent(10));  
    }  
  
    #[test]  
    fn handle_valid_purchase() {  
        // Create mocks  
        let mut deps = mock_dependencies(&[]);  
  
        // Create config state  
        instantiate(  
            deps.as_mut(),  
            mock_env(),  
            mock_info("feebucket", &[]),  
            InstantiateMsg {  
                contract_name: "tutorial.sc.pb".into(),  
                purchase_denom: "purchasecoin".into(),  
                merchant_address: Addr::unchecked("merchant"),  
                fee_percent: Decimal::percent(10),  
            },  
        )  
        .unwrap();  
  
        // Send a valid purchase message of 100purchasecoin  
        let res = execute(  
            deps.as_mut(),  
            mock_env(),  
            mock_info("consumer", &[coin(100, "purchasecoin")]),  
            ExecuteMsg::Purchase {  
                id: "a7918172-ac09-43f6-bc4b-7ac2fbad17e9".into(),  
            },  
        )  
        .unwrap();  
  
        // Ensure we have the merchant transfer and fee collection bank messages  
        assert_eq!(res.messages.len(), 2);  
  
        // Ensure we got the proper bank transfer values.  
        // 10% fees on 100 purchasecoin => 90 purchasecoin for the merchant and 10 purchasecoin for the fee bucket.        
        let expected_transfer = coin(90, "purchasecoin");  
        let expected_fees = coin(10, "purchasecoin");  
        res.messages.into_iter().for_each(|msg| match msg.msg {  
            CosmosMsg::Bank(BankMsg::Send {  
                amount, to_address, ..  
            }) => {  
                assert_eq!(amount.len(), 1);  
                if to_address == "merchant" {  
                    assert_eq!(amount[0], expected_transfer)  
                } else if to_address == "feebucket" {  
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
  
    #[test]  
    fn handle_invalid_funds() {  
        // Create mocks  
        let mut deps = mock_dependencies(&[]);  
  
        // Create config state  
        instantiate(  
            deps.as_mut(),  
            mock_env(),  
            mock_info("feebucket", &[]),  
            InstantiateMsg {  
                contract_name: "tutorial.sc.pb".into(),  
                purchase_denom: "purchasecoin".into(),  
                merchant_address: Addr::unchecked("merchant"),  
                fee_percent: Decimal::percent(10),  
            },  
        )  
        .unwrap();  
  
        // Don't send any funds  
        let err = execute(  
            deps.as_mut(),  
            mock_env(),  
            mock_info("consumer", &[]),  
            ExecuteMsg::Purchase {  
                id: "a7918172-ac09-43f6-bc4b-7ac2fbad17e9".into(),  
            },  
        )  
        .unwrap_err();  
  
        // Ensure the expected error was returned.  
        match err {  
            ContractError::Std(StdError::GenericErr { msg, .. }) => {  
                assert_eq!(msg, "no purchase funds sent")  
            }  
            _ => panic!("unexpected handle error"),  
        }  
  
        // Send zero amount for a valid denom  
        let err = execute(  
            deps.as_mut(),  
            mock_env(),  
            mock_info("consumer", &[coin(0, "purchasecoin")]),  
            ExecuteMsg::Purchase {  
                id: "a7918172-ac09-43f6-bc4b-7ac2fbad17e9".into(),  
            },  
        )  
        .unwrap_err();  
  
        // Ensure the expected error was returned.  
        match err {  
            ContractError::Std(StdError::GenericErr { msg, .. }) => {  
                assert_eq!(msg, "invalid purchase funds: 0purchasecoin")  
            }  
            _ => panic!("unexpected handle error"),  
        }  
  
        // Send invalid denom  
        let err = execute(  
            deps.as_mut(),  
            mock_env(),  
            mock_info("consumer", &[coin(100, "fakecoin")]),  
            ExecuteMsg::Purchase {  
                id: "a7918172-ac09-43f6-bc4b-7ac2fbad17e9".into(),  
            },  
        )  
        .unwrap_err();  
  
        // Ensure the expected error was returned.  
        match err {  
            ContractError::Std(StdError::GenericErr { msg, .. }) => {  
                assert_eq!(msg, "invalid purchase funds: 100fakecoin")  
            }  
            _ => panic!("unexpected handle error"),  
        }  
    }  
}
```

### JSON Schema

File: `src/bin/schema.rs`

Ensure a JSON schema is generated for the smart contract types.

```rust
use cosmwasm_schema::write_api;  
  
use tutorial::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};  
  
fn main() {  
    write_api! {  
        instantiate: InstantiateMsg,  
        execute: ExecuteMsg,  
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
