use cosmwasm_std::{Addr, DepsMut, Response};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError},
    events::change_owner::ChangeOwnerEvent,
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
/// let res = handle(deps, env, info.sender, msg.new_owner.as_slice())?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, new_owner: Addr) -> ProvTxResponse {
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
    use provwasm_mocks::mock_provenance_dependencies;

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
        let mut deps = mock_provenance_dependencies();
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
        let mut deps = mock_provenance_dependencies();
        let info = mock_info(false, CREATOR);
        let new_owner = Addr::unchecked(NEW_OWNER);

        setup::mock_contract(deps.as_mut());

        let err = handle(deps.as_mut(), info.sender, new_owner).unwrap_err();
        assert_eq!(ContractError::Unauthorized {}.to_string(), err.to_string());
    }
}
