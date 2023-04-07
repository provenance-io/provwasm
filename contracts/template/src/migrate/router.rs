use cosmwasm_std::Env;

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::MigrateMsg,
};

use super::v1_to_v2;

pub fn route(deps: &ProvDepsMut, env: Env, msg: MigrateMsg) -> ProvTxResponse {
    match msg {
        MigrateMsg::MigrateV1ToV2 {} => v1_to_v2::handle(deps, env, msg),
    }
}
