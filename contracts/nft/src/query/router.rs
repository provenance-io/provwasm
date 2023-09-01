use cosmwasm_std::{Binary, Deps, Env};

use crate::core::error::ContractError;
use crate::core::msg::QueryMsg;
use crate::query::nft_info;

pub fn route(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::NftInfo { id: token_id } => nft_info::handle(deps, env, token_id),
    }
}
