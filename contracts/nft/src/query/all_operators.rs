use cosmwasm_std::{to_binary, Deps, Env, Binary};
use crate::core::error::ContractError;

use crate::storage;

pub fn handle(
    deps: Deps,
    env: Env,
    owner: String,
    include_expired: bool,
    start_after: Option<String>,
    limit: Option<u32>,
) -> Result<Binary, ContractError> {
    let res = QueryOwnerResponse {
        owner: storage::state::get_owner(deps.storage)?,
    };
    Ok(to_binary(&res)?)
}

