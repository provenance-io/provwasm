use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};
use cw_utils::Expiration;

use crate::core::error::ContractError;
use crate::events::approve_all::EventApproveAll;
use crate::storage::operators::OPERATORS;
use crate::util::action::{Action, ActionType};

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    operator: Addr,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    // reject expired data as invalid
    let expires = expires.unwrap_or_default();
    if expires.is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }

    // set the operator for us
    OPERATORS.save(deps.storage, (&info.sender, &operator), &expires)?;

    Ok(Response::default()
        .set_action(ActionType::ApproveAll)
        .add_event(
            EventApproveAll {
                operator,
                sender: info.sender,
            }
            .into(),
        ))
}
