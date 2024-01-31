use cosmwasm_std::{to_json_binary, Binary, Deps, Order, StdResult};
use cw721::TokensResponse;
use cw_storage_plus::Bound;

use crate::core::constants::{DEFAULT_LIMIT, MAX_LIMIT};
use crate::core::error::ContractError;
use crate::storage::nft::TOKENS;

pub fn handle(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> Result<Binary, ContractError> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let owner_addr = deps.api.addr_validate(&owner)?;
    let tokens: Vec<String> = TOKENS
        .idx
        .owner
        .prefix(owner_addr)
        .keys(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(to_json_binary(&TokensResponse { tokens })?)
}
