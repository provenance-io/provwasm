use cosmwasm_std::{to_binary, Deps};

use crate::core::aliases::Result<Binary, ContractError>;
use crate::storage;

/// Performs the logic for the QueryOwner message and obtains the contract's owner.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
///
/// # Examples
/// ```
/// let res = handle(deps)?;
/// ```
pub fn handle(deps: Deps) -> Result<Binary, ContractError> {
    let res = QueryOwnerResponse {
        owner: storage::state::get_owner(deps.storage)?,
    };
    Ok(to_binary(&res)?)
}