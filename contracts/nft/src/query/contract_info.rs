use cosmwasm_std::{to_binary, Binary, Deps};

use crate::core::error::ContractError;
use crate::storage::contract_info::CONTRACT_INFO;

pub fn handle(deps: Deps) -> Result<Binary, ContractError> {
    Ok(to_binary(&CONTRACT_INFO.load(deps.storage)?)?)
}
