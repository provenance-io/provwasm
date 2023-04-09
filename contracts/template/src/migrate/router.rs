use cosmwasm_std::Env;

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::MigrateMsg,
};

use super::default;

pub fn route(deps: &ProvDepsMut, env: Env, msg: MigrateMsg) -> ProvTxResponse {
    match msg {
        MigrateMsg::Default {} => default::handle(deps, env, msg),
    }
}
