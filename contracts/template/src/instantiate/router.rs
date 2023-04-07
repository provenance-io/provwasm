use cosmwasm_std::{Env, MessageInfo};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::InstantiateMsg,
};

use super::instantiate;

pub fn route(
    deps: ProvDepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ProvTxResponse {
    match msg {
        InstantiateMsg::Instantiate { owner } => instantiate::handle(deps, owner),
    }
}
