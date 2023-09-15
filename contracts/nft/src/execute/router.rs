use std::str::FromStr;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use uuid::Uuid;

use crate::core::error::ContractError;
use crate::core::msg::ExecuteMsg;
use crate::execute::{
    approve, approve_all, burn, mint, revoke, revoke_all, send, transfer, update_ownership,
};

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
            recipient,
        } => mint::handle(
            deps,
            info,
            env,
            Uuid::from_str(scope_uuid.as_str()).unwrap(),
            Uuid::from_str(session_uuid.as_str()).unwrap(),
            recipient,
        ),
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires,
        } => approve::handle(deps, &env, &info, spender, &token_id, expires),
        ExecuteMsg::ApproveAll { operator, expires } => {
            approve_all::handle(deps, env, info, operator, expires)
        }
        ExecuteMsg::Revoke { spender, token_id } => {
            revoke::handle(deps, &env, &info, spender, &token_id)
        }
        ExecuteMsg::RevokeAll { operator } => revoke_all::handle(deps, env, info, operator),
        ExecuteMsg::TransferNft {
            token_id,
            recipient,
            session_uuid,
        } => transfer::handle(
            deps,
            env,
            info,
            recipient.clone(),
            Uuid::from_str(&token_id).unwrap(),
            Uuid::from_str(&session_uuid).unwrap(),
        ),
        ExecuteMsg::SendNft {
            token_id,
            session_uuid,
            contract,
            msg,
        } => send::handle(
            deps,
            env,
            info,
            Uuid::from_str(&token_id).unwrap(),
            Uuid::from_str(&session_uuid).unwrap(),
            contract,
            msg,
        ),
        ExecuteMsg::Burn { token_id: id } => {
            burn::handle(deps, env, info, Uuid::from_str(&id).unwrap())
        }
        ExecuteMsg::UpdateOwnership(action) => update_ownership::handle(deps, env, info, action),
    }
}
