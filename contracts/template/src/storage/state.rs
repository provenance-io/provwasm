use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Item;

use crate::{
    core::{constants::STATE_KEY, error::ContractError},
    util::{fee::Fee, state::State},
};

pub const STATE: Item<State> = Item::new(STATE_KEY);

pub fn get(storage: &dyn Storage) -> Result<State, ContractError> {
    Ok(STATE.load(storage)?)
}

pub fn set(storage: &mut dyn Storage, state: &State) -> Result<(), ContractError> {
    Ok(STATE.save(storage, state)?)
}

pub fn get_owner(storage: &dyn Storage) -> Result<Addr, ContractError> {
    let state = get(storage)?;
    Ok(state.owner)
}

pub fn get_fee(storage: &dyn Storage) -> Result<Fee, ContractError> {
    let state = get(storage)?;
    Ok(state.fee)
}

pub fn is_owner(storage: &dyn Storage, account: &Addr) -> Result<bool, ContractError> {
    let owner = get_owner(storage)?;
    Ok(owner == *account)
}

pub fn set_owner(storage: &mut dyn Storage, owner: Addr) -> Result<(), ContractError> {
    let mut state = get(storage)?;
    state.owner = owner;
    set(storage, &state)?;
    Ok(())
}
