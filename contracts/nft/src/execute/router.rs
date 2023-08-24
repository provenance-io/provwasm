use crate::core::error::ContractError;
use crate::core::msg::ExecuteMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use std::str::FromStr;
use uuid::Uuid;

use crate::execute::{burn, mint};

pub fn route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match &msg {
        ExecuteMsg::Mint {
            scope_uuid,
            session_uuid,
        } => mint::handle(
            deps,
            info,
            env,
            Uuid::from_str(scope_uuid.as_str()).unwrap(),
            Uuid::from_str(session_uuid.as_str()).unwrap(),
            msg.clone(),
        ),
        // ExecuteMsg::Approve {
        //     spender,
        //     token_id,
        //     expires,
        // } => approve::handle(deps, env, info, spender, token_id, expires),
        // ExecuteMsg::Revoke { spender, token_id } => {
        //     revoke::handle(deps, env, info, spender, token_id)
        // }
        // ExecuteMsg::ApproveAll { operator, expires } => {
        //     approve_all::handle(deps, env, info, operator, expires)
        // }
        // ExecuteMsg::RevokeAll { operator } => revoke_all::handle(deps, env, info, operator),
        // ExecuteMsg::TransferNft {
        //     recipient,
        //     token_id,
        // } => transfer::handle(deps, env, info, recipient, token_id),
        // ExecuteMsg::SendNft {
        //     contract,
        //     token_id,
        //     msg,
        // } => send::handle(deps, env, info, contract, token_id, msg),
        ExecuteMsg::Burn { id } => burn::handle(deps, env, info, id.clone()),
    }
}
