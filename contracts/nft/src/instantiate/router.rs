use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::core::error::ContractError;
use crate::core::msg::InstantiateMsg;

use super::default;

pub fn route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    match msg {
        InstantiateMsg::Default {} => default::handle(deps, env, info),
    }
}
