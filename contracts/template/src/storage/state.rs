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

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        storage::state::{get_fee, get_owner, is_owner, set, set_owner},
        util::{fee::Fee, state::State},
    };

    use super::get;

    #[test]
    fn test_invalid_get() {
        let deps = mock_dependencies(&[]);
        get(&deps.storage).unwrap_err();
    }

    #[test]
    fn test_get_set() {
        let mut deps = mock_dependencies(&[]);
        let expected = State::new(
            Addr::unchecked("addr1"),
            Fee {
                recipient: None,
                amount: Coin::new(0, "nhash"),
            },
        );
        set(deps.as_mut().storage, &expected).unwrap();
        let state = get(&deps.storage).unwrap();
        assert_eq!(expected, state);
    }

    #[test]
    fn test_get_owner() {
        let mut deps = mock_dependencies(&[]);
        let expected = State::new(
            Addr::unchecked("addr1"),
            Fee {
                recipient: None,
                amount: Coin::new(0, "nhash"),
            },
        );
        set(deps.as_mut().storage, &expected).unwrap();
        let owner = get_owner(&deps.storage).unwrap();
        assert_eq!(expected.owner, owner);
    }

    #[test]
    fn test_get_set_owner() {
        let mut deps = mock_dependencies(&[]);
        let state = State::new(
            Addr::unchecked("addr1"),
            Fee {
                recipient: None,
                amount: Coin::new(0, "nhash"),
            },
        );
        set(deps.as_mut().storage, &state).unwrap();
        let owner = get_owner(&deps.storage).unwrap();
        assert_eq!(state.owner, owner);
        set_owner(deps.as_mut().storage, Addr::unchecked("addr2")).unwrap();
        let owner = get_owner(&deps.storage).unwrap();
        assert_eq!(owner, Addr::unchecked("addr2"));
    }

    #[test]
    fn test_get_fee() {
        let mut deps = mock_dependencies(&[]);
        let expected = State::new(
            Addr::unchecked("addr1"),
            Fee {
                recipient: None,
                amount: Coin::new(0, "nhash"),
            },
        );
        set(deps.as_mut().storage, &expected).unwrap();
        let fee = get_fee(&deps.storage).unwrap();
        assert_eq!(expected.fee, fee);
    }

    #[test]
    fn test_is_owner_success() {
        let mut deps = mock_dependencies(&[]);
        let expected = State::new(
            Addr::unchecked("addr1"),
            Fee {
                recipient: None,
                amount: Coin::new(0, "nhash"),
            },
        );
        set(deps.as_mut().storage, &expected).unwrap();
        let is_owner = is_owner(&deps.storage, &Addr::unchecked("addr1")).unwrap();
        assert!(is_owner);
    }

    #[test]
    fn test_is_owner_failed() {
        let mut deps = mock_dependencies(&[]);
        let expected = State::new(
            Addr::unchecked("addr1"),
            Fee {
                recipient: None,
                amount: Coin::new(0, "nhash"),
            },
        );
        set(deps.as_mut().storage, &expected).unwrap();
        let is_owner = is_owner(&deps.storage, &Addr::unchecked("addr2")).unwrap();
        assert!(!is_owner);
    }
}
