use std::str::FromStr;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use uuid::Uuid;

use crate::core::error::ContractError;
use crate::core::msg::ExecuteMsg;
use crate::execute::{burn, mint, transfer};

pub fn route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {
            scope_uuid,
            session_uuid,
        } => mint::handle(
            deps,
            info,
            env,
            Uuid::from_str(scope_uuid.as_str()).unwrap(),
            Uuid::from_str(session_uuid.as_str()).unwrap(),
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
        ExecuteMsg::TransferNft {
            id,
            recipient,
            session_uuid,
        } => transfer::handle(
            deps,
            env,
            info,
            recipient.clone(),
            Uuid::from_str(id.as_str()).unwrap(),
            Uuid::from_str(session_uuid.as_str()).unwrap(),
        ),
        // ExecuteMsg::SendNft {
        //     contract,
        //     token_id,
        //     msg,
        // } => send::handle(deps, env, info, contract, token_id, msg),
        ExecuteMsg::Burn { id } => burn::handle(deps, env, info, Uuid::from_str(&id).unwrap()),
    }
}
