use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response};

use crate::core::error::ContractError;
use crate::core::msg::{ExecuteMsg, QueryMsg};
use crate::{core::msg::InstantiateMsg, execute, instantiate, query, util::validate::Validate};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    msg.validate(deps.as_ref())?;
    msg.validate_funds(&info.funds)?;
    instantiate::router::route(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    msg.validate(deps.as_ref())?;
    msg.validate_funds(&info.funds)?;
    execute::router::route(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    msg.validate(deps)?;
    query::router::route(deps, env, msg)
}
