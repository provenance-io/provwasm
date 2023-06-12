use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::core::error::ContractError;
use crate::{
    core::constants::{CONTRACT_NAME, CONTRACT_VERSION},
    util::action::{Action, ActionType},
};

pub fn handle(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&info.sender.as_str()))?;

    Ok(Response::default().set_action(ActionType::Initialize {}))
}
