use cosmwasm_std::{Env, MessageInfo};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::ExecuteMsg,
};

use super::change_owner;

pub fn route(deps: ProvDepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::ChangeOwner { new_owner } => change_owner::handle(deps, info.sender, new_owner),
    }
}
