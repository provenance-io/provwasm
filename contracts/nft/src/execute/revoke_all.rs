use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

use crate::core::error::ContractError;
use crate::storage::operators::OPERATORS;
use crate::util::action::{Action, ActionType};

pub fn handle(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    operator: Addr,
) -> Result<Response, ContractError> {
    OPERATORS.remove(deps.storage, (&info.sender, &operator));

    Ok(Response::default().set_action(ActionType::RevokeAll))
}
