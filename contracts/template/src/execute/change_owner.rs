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

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::error::ContractError,
        execute::change_owner::handle,
        testing::{
            constants::{CREATOR, NEW_OWNER, OWNER},
            setup::{self, mock_info},
        },
        util::action::ActionType,
    };

    #[test]
    fn test_handle_success() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(false, OWNER);
        let new_owner = Addr::unchecked(NEW_OWNER);

        setup::mock_contract(deps.as_mut());

        let res = handle(deps.as_mut(), info.sender, new_owner).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::ChangeOwner {})],
            res.attributes
        );
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_handle_is_not_owner() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info(false, CREATOR);
        let new_owner = Addr::unchecked(NEW_OWNER);

        setup::mock_contract(deps.as_mut());

        let err = handle(deps.as_mut(), info.sender, new_owner).unwrap_err();
        assert_eq!(ContractError::Unauthorized {}.to_string(), err.to_string());
    }
}
