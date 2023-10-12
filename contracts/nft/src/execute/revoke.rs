use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

use crate::core::error::ContractError;
use crate::events::revoke::EventRevoke;
use crate::util::action::{Action, ActionType};
use crate::util::permission;

pub fn handle(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    spender: Addr,
    token_id: &str,
) -> Result<Response, ContractError> {
    permission::modify_approvals(deps, env, info, spender.clone(), token_id, false, None)?;

    Ok(Response::default()
        .set_action(ActionType::Revoke)
        .add_event(EventRevoke { spender, token_id }.into()))
}
