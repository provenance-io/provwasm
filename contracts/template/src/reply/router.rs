use cosmwasm_std::{Env, Reply};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    constants::DEFAULT_REPLY,
    error::ContractError,
};

use super::default;

pub fn route(deps: ProvDepsMut, env: Env, reply: Reply) -> ProvTxResponse {
    match reply.id {
        DEFAULT_REPLY => default::handle(deps, env, reply),
        _ => Err(ContractError::UnexpectedReplyId(reply.id)),
    }
}
