use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo};

use crate::{
    core::{
        aliases::{ProvQueryResponse, ProvTxResponse},
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    },
    execute, instantiate,
    migrate::{self},
    query,
    util::validate::Validate,
};

/// An entry point that is triggered when a user instantiates an instance of the stored wasm code.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `info` - Contains the message signer and any sent funds.
/// * `msg` - The message enum and its contents used to trigger this endpoint.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ProvTxResponse {
    msg.validate(deps.as_ref())?;
    msg.validate_funds(&info.funds)?;
    instantiate::router::route(deps, env, info, msg)
}

/// An entry point that is triggered when a user queries the contract.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `msg` - The message enum and its contents used to trigger this endpoint.
#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ProvQueryResponse {
    msg.validate(deps)?;
    query::router::route(deps, env, msg)
}

/// An entry point that is triggered when a user runs one of the exposed execute messages.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `info` - Contains the message signer and any sent funds.
/// * `msg` - The message enum and its contents used to trigger this endpoint.
#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    msg.validate(deps.as_ref())?;
    msg.validate_funds(&info.funds)?;
    execute::router::route(deps, env, info, msg)
}

/// An entry point that is triggered when an admin attempts to migrate the contract.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `msg` - The message enum and its contents used to trigger this endpoint.
#[entry_point]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> ProvTxResponse {
    msg.validate(deps.as_ref())?;
    let res = migrate::router::route(&deps, env, msg);
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    res
}
