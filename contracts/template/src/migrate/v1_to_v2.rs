use cosmwasm_std::{Env, Response};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::MigrateMsg,
};

pub fn handle(_deps: &ProvDepsMut, _env: Env, _msg: MigrateMsg) -> ProvTxResponse {
    Ok(Response::new())
}
