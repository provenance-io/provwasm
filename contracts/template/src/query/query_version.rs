use cosmwasm_std::to_binary;
use cw2::get_contract_version;

use crate::core::{
    aliases::{ProvDeps, ProvQueryResponse},
    msg::QueryVersionResponse,
};

pub fn handle(deps: ProvDeps) -> ProvQueryResponse {
    let res = QueryVersionResponse {
        contract_version: get_contract_version(deps.storage)?,
    };
    Ok(to_binary(&res)?)
}
