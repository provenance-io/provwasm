use cosmwasm_std::{Addr, DepsMut, Response};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError},
    events::set_tag::SetTagEvent,
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
/// * `asset_addr` - The address of the asset to set the tag for.
/// * `tag` - The label to set as the asset's tag.
///
/// # Examples
/// ```
/// let msg = ExecuteMsg::SetTag {asset_addr: Addr::unchecked("addr"), tag: "tag"};
/// let res = handle(deps, env, info.sender, msg.asset_addr, &msg.tag)?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, asset_addr: Addr, tag: &str) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    storage::tag::set_asset_tag(deps.storage, &asset_addr, tag)?;

    // We need to emit the event
    Ok(Response::default()
        .set_action(ActionType::SetTag {})
        .add_event(SetTagEvent::new(asset_addr, tag).into()))
}
