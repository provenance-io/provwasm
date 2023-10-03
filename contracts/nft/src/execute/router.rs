use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::core::error::ContractError;
use crate::core::msg::ExecuteMsg;
use crate::execute::{
    approve, approve_all, burn, mint, revoke, revoke_all, send, transfer, update_ownership,
};
use crate::util::parse_uuid;

pub fn route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {
            token_id,
            session_uuid,
            recipient,
        } => mint::handle(
            deps,
            info,
            env,
            parse_uuid(&token_id)?,
            parse_uuid(&session_uuid)?,
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
            parse_uuid(&token_id)?,
            parse_uuid(&session_uuid)?,
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
            parse_uuid(&token_id)?,
            parse_uuid(&session_uuid)?,
            contract,
            msg,
        ),
        ExecuteMsg::Burn { token_id } => burn::handle(deps, env, info, parse_uuid(&token_id)?),
        ExecuteMsg::UpdateOwnership(action) => update_ownership::handle(deps, env, info, action),
    }
}
