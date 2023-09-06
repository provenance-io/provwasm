use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_utils::Expiration;

use crate::core::error::ContractError;
use crate::storage::operators::OPERATORS;
use crate::util::action::{Action, ActionType};

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    operator: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    // reject expired data as invalid
    let expires = expires.unwrap_or_default();
    if expires.is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }

    // set the operator for us
    let operator_addr = deps.api.addr_validate(&operator)?;
    OPERATORS.save(deps.storage, (&info.sender, &operator_addr), &expires)?;

    Ok(Response::default().set_action(ActionType::ApproveAll))
}
