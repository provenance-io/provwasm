use crate::core::error::ContractError;
use cosmwasm_std::{to_json_binary, Binary, Deps};
use cw2::get_contract_version;

use crate::core::msg::ContractVersionResponse;

/// Performs the logic for querying the contract version
///
/// # Arguments
///
/// * `deps` -  dependencies from the contract entrypoint
///
pub fn handle(deps: Deps) -> Result<Binary, ContractError> {
    let res = ContractVersionResponse {
        contract_version: get_contract_version(deps.storage)?,
    };
    Ok(to_json_binary(&res)?)
}
