use cosmwasm_std::{to_json_binary, Binary, Deps, Env, Order, StdResult};
use cw721::OperatorsResponse;
use cw_storage_plus::Bound;
use cw_utils::maybe_addr;

use crate::core::constants::{DEFAULT_LIMIT, MAX_LIMIT};
use crate::core::error::ContractError;
use crate::storage::operators::OPERATORS;
use crate::util::permission::parse_approval;

pub fn handle(
    deps: Deps,
    env: Env,
    owner: String,
    include_expired: bool,
    start_after: Option<String>,
    limit: Option<u32>,
) -> Result<Binary, ContractError> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_addr = maybe_addr(deps.api, start_after)?;
    let start = start_addr.as_ref().map(Bound::exclusive);

    let owner_addr = deps.api.addr_validate(&owner)?;
    let res: StdResult<Vec<_>> = OPERATORS
        .prefix(&owner_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .filter(|r| include_expired || r.is_err() || !r.as_ref().unwrap().1.is_expired(&env.block))
        .take(limit)
        .map(parse_approval)
        .collect();
    Ok(to_json_binary(&OperatorsResponse { operators: res? })?)
}
