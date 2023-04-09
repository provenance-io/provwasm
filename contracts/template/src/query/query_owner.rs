use cosmwasm_std::to_binary;

use crate::{
    core::{
        aliases::{ProvDeps, ProvQueryResponse},
        msg::QueryOwnerResponse,
    },
    storage,
};

pub fn handle(deps: ProvDeps) -> ProvQueryResponse {
    let res = QueryOwnerResponse {
        owner: storage::state::get_owner(deps.storage)?,
    };
    Ok(to_binary(&res)?)
}
