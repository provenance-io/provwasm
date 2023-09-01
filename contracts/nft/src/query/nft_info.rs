use cosmwasm_std::{to_binary, Binary, Deps, Env};

use crate::core::error::ContractError;
use crate::core::msg::NftInfoResponse;
use crate::storage::nft::TOKENS;

pub fn handle(deps: Deps, _env: Env, token_id: String) -> Result<Binary, ContractError> {
    let info = TOKENS.load(deps.storage, &token_id)?;
    Ok(to_binary(&NftInfoResponse {
        id: info.id,
        owner: info.owner,
    })?)
}
