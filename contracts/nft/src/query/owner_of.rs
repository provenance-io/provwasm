use cosmwasm_std::{to_binary, Binary, Deps, Env};
use cw721::OwnerOfResponse;

use crate::core::error::ContractError;
use crate::storage::nft::TOKENS;
use crate::util::permission::humanize_approvals;

pub fn handle(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: bool,
) -> Result<Binary, ContractError> {
    let info = TOKENS.load(deps.storage, &token_id)?;
    Ok(to_binary(&OwnerOfResponse {
        owner: info.owner.to_string(),
        approvals: humanize_approvals(&env.block, &info, include_expired),
    })?)
}
