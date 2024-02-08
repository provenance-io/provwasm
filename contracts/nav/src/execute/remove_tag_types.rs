use cosmwasm_std::{Addr, DepsMut, Response};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError},
    events::update_tag_types::UpdateTagTypesEvent,
    storage::{self, asset},
    util::action::{Action, ActionType},
};

/// Performs the execute logic for the RemoveTagTypes variant of ExecuteMsg.
///
/// Removes specified tag types from the contract's store.
/// The sender must be the owner and each specified tag type
/// must not be in use to succeed.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `tag_types` - The tags to be removed.
///
/// # Examples
/// ```
/// let res = handle(deps, env, info.sender, msg.tags)?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, tag_types: &[String]) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    for tag in tag_types {
        if asset::has_tag(deps.storage, tag) {
            return Err(ContractError::TagInUse(tag.clone()));
        }

        storage::tag::remove_type(deps.storage, tag);
    }

    Ok(Response::default()
        .set_action(ActionType::RemoveTagTypes {})
        .add_event(UpdateTagTypesEvent::new().into()))
}
