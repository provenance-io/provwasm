use cosmwasm_std::{to_binary, Binary, Deps, Env};
use provwasm_std::types::provenance::metadata::v1::MetadataQuerier;

use crate::core::error::ContractError;
use crate::core::msg::NftData;
use crate::storage::nft::TOKENS;

pub fn handle(deps: Deps, _env: Env, token_id: String) -> Result<Binary, ContractError> {
    let info = TOKENS.load(deps.storage, &token_id)?;

    let scope_response = MetadataQuerier::new(&deps.querier).scope(
        token_id,
        "".to_string(),
        "".to_string(),
        true,
        true,
    )?;

    Ok(to_binary(&cw721::NftInfoResponse {
        token_uri: None,
        extension: &NftData {
            id: info.id,
            owner: info.owner,
            data: scope_response,
        },
    })?)
}
