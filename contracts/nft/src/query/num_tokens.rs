use crate::core::error::ContractError;
use cosmwasm_std::{to_binary, Binary, Deps};
use cw721::NumTokensResponse;

use crate::storage::nft_count::NFT_COUNT;

pub fn handle(deps: Deps) -> Result<Binary, ContractError> {
    let count = NFT_COUNT.may_load(deps.storage)?.unwrap_or_default();
    Ok(to_binary(&NumTokensResponse { count })?)
}
