use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};
use cw_utils::Expiration;

use crate::core::error::ContractError;
use crate::events::approve::EventApprove;
use crate::util::action::{Action, ActionType};
use crate::util::permission;

pub fn handle(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    spender: Addr,
    token_id: &str,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    permission::modify_approvals(deps, env, info, spender.clone(), token_id, true, expires)?;

    Ok(Response::default()
        .set_action(ActionType::Approve)
        .add_event(EventApprove { spender, token_id }.into()))
}
