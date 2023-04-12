use cosmwasm_std::{Addr, Env, Response};
use cw2::set_contract_version;
use provwasm_std::assess_custom_fee;

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
    },
    storage,
    util::{
        action::{Action, ActionType},
        fee::Fee,
        state::State,
    },
};

pub fn handle(deps: ProvDepsMut, env: Env, owner: Addr, fee: Fee) -> ProvTxResponse {
    storage::state::set(deps.storage, &State::new(owner, fee.clone()))?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let fee_message = assess_custom_fee(
        fee.amount.clone(),
        Some("contract_fee"),
        env.contract.address,
        fee.recipient,
    )?;
    Ok(Response::default()
        .set_action(ActionType::Initialize {})
        .add_message(fee_message))
}
