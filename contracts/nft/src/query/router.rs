use cosmwasm_std::{Binary, Deps, Env};

use crate::core::error::ContractError;
use crate::core::msg::QueryMsg;
use crate::query::{
    all_nft_info, all_operators, all_tokens, approval, approvals, contract_info, contract_owner,
    contract_version, minter, nft_info, num_tokens, operator, owner_of, tokens,
};

pub fn route(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::NftInfo { token_id } => nft_info::handle(deps, env, token_id),
        QueryMsg::OwnerOf {
            token_id,
            include_expired,
        } => owner_of::handle(deps, env, token_id, include_expired.unwrap_or(false)),
        QueryMsg::Approval {
            token_id,
            spender,
            include_expired,
        } => approval::handle(
            deps,
            env,
            token_id,
            spender,
            include_expired.unwrap_or(false),
        ),
        QueryMsg::Approvals {
            token_id,
            include_expired,
        } => approvals::handle(deps, env, token_id, include_expired.unwrap_or(false)),
        QueryMsg::Operator {
            owner,
            operator,
            include_expired,
        } => operator::handle(deps, env, owner, operator, include_expired.unwrap_or(false)),
        QueryMsg::AllOperators {
            owner,
            include_expired,
            start_after,
            limit,
        } => all_operators::handle(
            deps,
            env,
            owner,
            include_expired.unwrap_or(false),
            start_after,
            limit,
        ),
        QueryMsg::NumTokens {} => num_tokens::handle(deps),
        QueryMsg::ContractInfo {} => contract_info::handle(deps),
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => all_nft_info::handle(deps, env, token_id, include_expired.unwrap_or(false)),
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => tokens::handle(deps, owner, start_after, limit),
        QueryMsg::AllTokens { start_after, limit } => all_tokens::handle(deps, start_after, limit),
        QueryMsg::Minter {} => minter::handle(deps),
        QueryMsg::ContractVersion {} => contract_version::handle(deps),
        QueryMsg::Ownership {} => contract_owner::handle(deps),
    }
}
