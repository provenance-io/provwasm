use cosmwasm_std::{Addr, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        error::ContractError,
    },
    events::change_owner::ChangeOwnerEvent,
    storage,
    util::action::{Action, ActionType},
};

pub fn handle(deps: ProvDepsMut, sender: Addr, new_owner: Addr) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    storage::state::set_owner(deps.storage, new_owner.clone())?;

    Ok(Response::default()
        .set_action(ActionType::ChangeOwner {})
        .add_event(ChangeOwnerEvent::new(sender, new_owner).into()))
}
