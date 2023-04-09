use cosmwasm_std::{Env, Reply, Response};

use crate::core::aliases::{ProvDepsMut, ProvTxResponse};

pub fn handle(_deps: ProvDepsMut, _env: Env, _reply: Reply) -> ProvTxResponse {
    Ok(Response::default())
}
