use cosmwasm_std::{to_binary, Env, Deps};

use crate::core::aliases::Result<Binary, ContractError>;
use crate::core::{
    aliases::{ProvDeps, ProvQueryResponse},
    msg::QueryMsg,
};
use crate::query::{all_nft_info, all_operators, all_tokens, approval, approvals, contract_info, nft_info, num_tokens, operator, tokens};

use super::{minter, query_version};

/// Routes the query message to the appropriate handler based on the message's variant.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `msg` - The query being ran by the user.
///
/// # Examples
/// ```
/// let msg = QueryMsg::QueryVersion {};
/// let res = route(deps, env, msg)?;
/// ```
pub fn route(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::ContractInfo {} => contract_info::handle(deps),
        QueryMsg::NftInfo { token_id } => nft_info::handle(deps, token_id),
        QueryMsg::OwnerOf {
            token_id,
            include_expired,
        } => owner_of(deps, env, token_id, include_expired.unwrap_or(false)),
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => {
            all_nft_info::handle(deps, env, token_id, include_expired.unwrap_or(false))
        }
        QueryMsg::Operator {
            owner,
            operator,
            include_expired,
        } => operator::handle(
            deps,
            env,
            owner,
            operator,
            include_expired.unwrap_or(false),
        ),
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
        )?),
        QueryMsg::NumTokens {} => num_tokens::handle(deps),
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => tokens::handle(deps, owner, start_after, limit),
        QueryMsg::AllTokens { start_after, limit } => {
            all_tokens::handle(deps, start_after, limit)
        }
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
    }
}
