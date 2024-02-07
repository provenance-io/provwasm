use cosmwasm_std::{Addr, DepsMut, Response};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError},
    storage,
};

/// Performs the execute logic for the AddTagTypes variant of ExecuteMsg.
///
/// If the sender is the owner of the contract, then the contract will update the contract's
/// accepted tag types.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `tag_types` - The tags to be added.
///
/// # Examples
/// ```
/// let res = handle(deps, env, info.sender, msg.tags)?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, tag_types: Vec<String>) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    // We need to emit the event
    Ok(Response::default())
}
