use cosmwasm_std::{to_binary, Binary, Deps};
use cw721::NumTokensResponse;

use crate::core::error::ContractError;
use crate::storage::nft_count::nft_count;

pub fn handle(deps: Deps) -> Result<Binary, ContractError> {
    let count = nft_count(deps.storage)?;
    Ok(to_binary(&NumTokensResponse { count })?)
}
