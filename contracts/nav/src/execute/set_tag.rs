use cosmwasm_std::{Addr, DepsMut, Response};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError},
    storage,
    util::action::{Action, ActionType},
};

/// Performs the execute logic for the ChangeOwner variant of ExecuteMsg.
///
/// If the sender is the owner of the contract, then the contract will update its owner
/// to the address of new_owner.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `new_owner` - The address of the contract's new owner.
///
/// # Examples
/// ```
/// let msg = ExecuteMsg::ChangeOwner {new_owner: Addr::unchecked("new_owner")};
/// let res = handle(deps, env, info.sender, msg.new_owner)?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, asset_addr: Addr, tag: String) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::default().set_action(ActionType::SetTag {}))
}
