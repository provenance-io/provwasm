use cosmwasm_std::{Addr, DepsMut, Response};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError},
    events::remove_security::RemoveSecurityEvent,
    storage,
    util::action::{Action, ActionType},
};

/// Performs the execute logic for the RemoveSecurity variant of ExecuteMsg.
///
/// If the sender is the owner of the contract, then the contract will remove the
/// security from the asset.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `asset_addr` - The address of the asset to remove the security from.
///
/// # Examples
/// ```
/// let msg = ExecuteMsg::RemoveSecurity {asset_addr: Addr::unchecked("addr")};
/// let res = handle(deps, env, info.sender, msg.asset_addr)?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, asset_addr: Addr) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    let security = storage::asset::get_security(deps.storage, &asset_addr)?;
    storage::asset::remove_security(deps.storage, &asset_addr);

    Ok(Response::default()
        .set_action(ActionType::RemoveSecurity {})
        .add_event(RemoveSecurityEvent::new(&asset_addr, &security).into()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{error::ContractError, msg::Security},
        events::remove_security::RemoveSecurityEvent,
        execute::remove_security::handle,
        storage,
        testing::{
            constants::{OWNER, TAG1},
            setup,
        },
        util::action::ActionType,
    };

    #[test]
    fn test_must_be_owner() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked("invalid");
        let asset_addr = Addr::unchecked("asset");

        setup::mock_contract(deps.as_mut());

        let error = handle(deps.as_mut(), sender, asset_addr).expect_err("should return an error");

        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_removes_tag() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("marker");
        let expected_events: Vec<Event> =
            vec![RemoveSecurityEvent::new(&asset_addr, &Security::new(TAG1)).into()];

        setup::mock_contract(deps.as_mut());

        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &Security::new(TAG1))
            .expect("should set security");
        let res =
            handle(deps.as_mut(), sender, asset_addr.clone()).expect("should not return an error");
        let found = storage::asset::has_security(&deps.storage, &Security::new(TAG1));

        assert_eq!(false, found);
        assert_eq!(
            vec![Attribute::from(ActionType::RemoveSecurity {})],
            res.attributes
        );
        assert_eq!(expected_events, res.events);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_removes_tag_returns_error_on_non_existing_asset() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("marker");

        setup::mock_contract(deps.as_mut());

        handle(deps.as_mut(), sender, asset_addr.clone()).expect_err("should return an error");

        let found = storage::asset::has_security(&deps.storage, &Security::new(TAG1));
        assert_eq!(false, found);
    }
}
