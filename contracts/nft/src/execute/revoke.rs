use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

use crate::core::error::ContractError;
use crate::util::action::{Action, ActionType};
use crate::util::permission;

pub fn handle(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    spender: Addr,
    token_id: &str,
) -> Result<Response, ContractError> {
    permission::modify_approvals(deps, env, info, spender, token_id, false, None)?;

    Ok(Response::default().set_action(ActionType::Revoke))
}
