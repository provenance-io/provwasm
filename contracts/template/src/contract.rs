use cosmwasm_std::{entry_point, Env, MessageInfo};

use crate::{
    core::{
        aliases::{ProvDeps, ProvDepsMut, ProvQueryResponse, ProvTxResponse},
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    },
    execute,
    types::validate::Validate,
};

/*
#[entry_point]
pub fn instantiate(
    deps: ProvDepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ProvTxResponse {
    msg.validate()?;
    msg.validate_funds(&info.funds)?;
    instantiate::router::route(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: ProvDeps, env: Env, msg: QueryMsg) -> ProvQueryResponse {
    msg.validate()?;
    query::router::route(deps, env, msg)
}
*/

#[entry_point]
pub fn execute(deps: ProvDepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    msg.validate()?;
    msg.validate_funds(&info.funds)?;
    execute::router::route(deps, env, info, msg)
}

/*
#[entry_point]
pub fn migrate(deps: ProvDepsMut, env: Env, msg: MigrateMsg) -> ProvTxResponse {
    msg.validate()?;
    let res = migrate::router::route(&deps, env, msg)
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    res
}
*/
