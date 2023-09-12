use cosmwasm_std::{to_binary, Binary, Deps, Order, StdResult};
use cw721::TokensResponse;
use cw_storage_plus::Bound;

use crate::core::constants::{DEFAULT_LIMIT, MAX_LIMIT};
use crate::core::error::ContractError;
use crate::storage::nft::TOKENS;

pub fn handle(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> Result<Binary, ContractError> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let tokens: StdResult<Vec<String>> = TOKENS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(k, _)| k))
        .collect();

    Ok(to_binary(&TokensResponse { tokens: tokens? })?)
}
