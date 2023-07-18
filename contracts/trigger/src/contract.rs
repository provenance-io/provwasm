use cosmwasm_std::{
    entry_point, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InitMsg, QueryMsg};

/// Initialize the smart contract
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> Result<Response, ContractError> {
    let res = Response::new()
        .add_attribute("integration_test", "v2")
        .add_attribute("action", "provwasm.contracts.trigger.init");
    Ok(res)
}

/// Handle messages that create or delete triggers.
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateTrigger {} => create_trigger(deps, env, info),
        ExecuteMsg::DeleteTrigger {} => delete_trigger(deps, env, info),
    }
}

// create a trigger that executes at a later time
pub fn create_trigger(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Err(ContractError::Std(StdError::GenericErr {
        msg: "not implemented".to_string(),
    }))
}

// delete an existing trigger
pub fn delete_trigger(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Err(ContractError::Std(StdError::GenericErr {
        msg: "not implemented".to_string(),
    }))
}

/// Handle query requests for the provenance trigger module
#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    Err(ContractError::Std(StdError::GenericErr {
        msg: "not implemented".to_string(),
    }))
}
