use crate::core::error::ContractError;
use crate::core::msg::MinterResponse;
use cosmwasm_std::{to_json_binary, Binary, Deps};

pub fn handle(deps: Deps) -> Result<Binary, ContractError> {
    let minter = cw_ownable::get_ownership(deps.storage)?
        .owner
        .map(|a| a.into_string());

    Ok(to_json_binary(&MinterResponse { minter })?)
}
