use cosmwasm_std::{to_binary, Binary, Deps, Env};
use cw721::{AllNftInfoResponse, NftInfoResponse, OwnerOfResponse};

use crate::core::error::ContractError;
use crate::core::msg::NftData;
use crate::query::nft_info::load_scope;
use crate::storage::nft::TOKENS;
use crate::util::permission::humanize_approvals;

pub fn handle(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: bool,
) -> Result<Binary, ContractError> {
    let info = TOKENS.load(deps.storage, &token_id)?;
    Ok(to_binary(&AllNftInfoResponse {
        access: OwnerOfResponse {
            owner: info.owner.to_string(),
            approvals: humanize_approvals(&env.block, &info, include_expired),
        },
        info: NftInfoResponse {
            token_uri: None,
            extension: &NftData {
                id: info.id,
                owner: info.owner,
                data: load_scope(token_id, deps.querier)?,
            },
        },
    })?)
}
