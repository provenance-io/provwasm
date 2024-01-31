use cosmwasm_std::{to_json_binary, Binary, Deps, Env};
use cw721::ApprovalsResponse;

use crate::core::error::ContractError;
use crate::storage::nft::TOKENS;

pub fn handle(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: bool,
) -> Result<Binary, ContractError> {
    let token = TOKENS.load(deps.storage, &token_id)?;
    let approvals: Vec<_> = token
        .approvals
        .into_iter()
        .filter(|t| include_expired || !t.is_expired(&env.block))
        .map(|a| cw721::Approval {
            spender: a.spender.into_string(),
            expires: a.expires,
        })
        .collect();

    Ok(to_json_binary(&ApprovalsResponse { approvals })?)
}
