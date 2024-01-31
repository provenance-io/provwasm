use cosmwasm_std::{to_json_binary, Binary, Deps};

use crate::core::error::ContractError;

pub fn handle(deps: Deps) -> Result<Binary, ContractError> {
    Ok(to_json_binary(&cw_ownable::get_ownership(deps.storage)?)?)
}
