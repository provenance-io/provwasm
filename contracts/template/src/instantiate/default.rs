use cosmwasm_std::{Addr, Response};

use crate::{
    core::aliases::{ProvDepsMut, ProvTxResponse},
    storage,
    util::{
        action::{Action, ActionType},
        fee::Fee,
        state::State,
    },
};

pub fn handle(deps: ProvDepsMut, owner: Addr, fee: Fee) -> ProvTxResponse {
    storage::state::set(deps.storage, &State::new(owner, fee))?;
    Ok(Response::default().set_action(ActionType::Initialize {}))
}
