use cosmwasm_std::{Binary, Deps, Env};

use crate::core::error::ContractError;
use crate::core::msg::QueryMsg;
use crate::query::nft_info;

pub fn route(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::NftInfo { token_id } => nft_info::handle(deps, env, token_id),
        QueryMsg::OwnerOf {
            token_id,
            include_expired,
        } => {}
        QueryMsg::Approval {
            token_id,
            spender,
            include_expired,
        } => {}
        QueryMsg::Approvals {
            token_id,
            include_expired,
        } => {}
        QueryMsg::Operator {
            owner,
            operator,
            include_expired,
        } => {}
        QueryMsg::AllOperators {
            owner,
            include_expired,
            start_after,
            limit,
        } => {}
        QueryMsg::NumTokens {} => {}
        QueryMsg::ContractInfo {} => {}
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => {}
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => {}
        QueryMsg::AllTokens { start_after, limit } => {}
        QueryMsg::Minter {} => {}
    }
}
