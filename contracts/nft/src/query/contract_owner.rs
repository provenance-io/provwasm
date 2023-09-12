use cosmwasm_std::{to_binary, Binary, Deps};

use crate::core::error::ContractError;

pub fn handle(deps: Deps) -> Result<Binary, ContractError> {
    Ok(to_binary(&cw_ownable::get_ownership(deps.storage)?)?)
}
