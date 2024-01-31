use cosmwasm_std::{to_json_binary, Binary, Deps};

use crate::core::error::ContractError;
use crate::storage::contract_info::CONTRACT_INFO;

pub fn handle(deps: Deps) -> Result<Binary, ContractError> {
    Ok(to_json_binary(&CONTRACT_INFO.load(deps.storage)?)?)
}
