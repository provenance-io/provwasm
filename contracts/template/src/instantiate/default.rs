use cosmwasm_std::{Addr, Env, Response};
use provwasm_std::assess_custom_fee;

use crate::{
    core::aliases::{ProvDepsMut, ProvTxResponse},
    storage,
    util::{
        action::{Action, ActionType},
        fee::Fee,
        state::State,
    },
};

pub fn handle(deps: ProvDepsMut, env: Env, owner: Addr, fee: Fee) -> ProvTxResponse {
    storage::state::set(deps.storage, &State::new(owner, fee.clone()))?;
    let fee_message = assess_custom_fee(
        fee.amount.clone(),
        Some("contract fee"),
        env.contract.address,
        fee.recipient,
    )?;
    Ok(Response::default()
        .set_action(ActionType::Initialize {})
        .add_message(fee_message))
}
