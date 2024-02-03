use cosmwasm_std::{to_json_binary, Binary, Deps, Env, QuerierWrapper, StdError};
use cw721::NftInfoResponse;
use provwasm_std::types::provenance::metadata::v1::{MetadataQuerier, ScopeResponse};

use crate::core::error::ContractError;
use crate::core::msg::NftData;
use crate::storage::nft::TOKENS;

pub fn handle(deps: Deps, _env: Env, token_id: String) -> Result<Binary, ContractError> {
    let info = TOKENS.load(deps.storage, &token_id)?;

    Ok(to_json_binary(&NftInfoResponse {
        token_uri: None,
        extension: &NftData {
            id: info.id,
            owner: info.owner,
            data: load_scope(token_id, deps.querier)?,
        },
    })?)
}

pub fn load_scope(scope_id: String, querier: QuerierWrapper) -> Result<ScopeResponse, StdError> {
    MetadataQuerier::new(&querier).scope(
        scope_id,
        "".to_string(),
        "".to_string(),
        true,
        true,
        false,
        false,
    )
}
