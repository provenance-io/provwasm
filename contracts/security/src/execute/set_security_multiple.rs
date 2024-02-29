use cosmwasm_std::{Addr, DepsMut, Response, Uint64};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError, msg::Security},
    events::set_security_multiple::SetSecurityMultipleEvent,
    storage,
    util::action::{Action, ActionType},
};

use super::set_security::set_security;

/// Performs the execute logic for the SetSecurityMultiple variant of ExecuteMsg.
///
/// If the sender is the owner of the contract, then the contract will update the security
/// for the asset. Every updated asset must be a marker or a scope.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `assets` - The addresses of the assets to set the security for.
/// * `security` - The security to link to the asset.
///
/// # Examples
/// ```
/// let msg = ExecuteMsg::SetSecurityMultiple {assets: vec![Addr::unchecked("addr")], security: Security::new("tag")};
/// let res = handle(deps, env, info.sender, &msg.assets, &msg.security)?;
/// ```
pub fn handle(
    mut deps: DepsMut,
    sender: Addr,
    assets: &[Addr],
    security: &Security,
) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    for asset in assets {
        set_security(&mut deps, asset, security)?;
    }

    Ok(Response::default()
        .set_action(ActionType::SetSecurityMultiple {})
        .add_event(
            SetSecurityMultipleEvent::new(Uint64::new(assets.len() as u64), security).into(),
        ))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event, Uint64};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{error::ContractError, msg::Security},
        events::set_security_multiple::SetSecurityMultipleEvent,
        storage,
        testing::{
            constants::{OWNER, TAG1},
            setup,
        },
        util::action::ActionType,
    };

    use super::handle;

    #[test]
    fn test_adds_security_for_single_entry() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let assets = vec![Addr::unchecked("marker")];
        let security = Security::new(TAG1);
        let expected_events: Vec<Event> =
            vec![SetSecurityMultipleEvent::new(Uint64::new(1), &security).into()];
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let res =
            handle(deps.as_mut(), sender, &assets, &security).expect("should not return an error");

        for asset in &assets {
            let stored_security = storage::asset::get_security(&deps.storage, asset)
                .expect("should have security for asset");
            assert_eq!(security, stored_security);
        }

        assert_eq!(
            vec![Attribute::from(ActionType::SetSecurityMultiple {})],
            res.attributes
        );
        assert_eq!(expected_events, res.events);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_can_handle_empty() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let assets = vec![];
        let security = Security::new(TAG1);
        let expected_events: Vec<Event> =
            vec![SetSecurityMultipleEvent::new(Uint64::new(0), &security).into()];
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let res =
            handle(deps.as_mut(), sender, &assets, &security).expect("should not return an error");

        assert_eq!(
            vec![Attribute::from(ActionType::SetSecurityMultiple {})],
            res.attributes
        );
        assert_eq!(expected_events, res.events);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_adds_security_for_multiple_entry() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let assets = vec![Addr::unchecked("marker"), Addr::unchecked("scope")];
        let security = Security::new(TAG1);
        let expected_events: Vec<Event> =
            vec![SetSecurityMultipleEvent::new(Uint64::new(2), &security).into()];
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let res =
            handle(deps.as_mut(), sender, &assets, &security).expect("should not return an error");

        for asset in &assets {
            let stored_security = storage::asset::get_security(&deps.storage, asset)
                .expect("should have security for asset");
            assert_eq!(security, stored_security);
        }

        assert_eq!(
            vec![Attribute::from(ActionType::SetSecurityMultiple {})],
            res.attributes
        );
        assert_eq!(expected_events, res.events);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_fails_for_invalid_addr() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let assets = vec![Addr::unchecked("invalid")];
        let security: Security = Security::new("");
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let err = handle(deps.as_mut(), sender, &assets, &security)
            .expect_err("should not return an error");

        assert_eq!(
            ContractError::AssetDoesNotExist(assets[0].to_string()).to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_fails_for_invalid_security() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let assets = vec![Addr::unchecked("marker")];
        let security: Security = Security::new("invalid");
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let err = handle(deps.as_mut(), sender, &assets, &security)
            .expect_err("should not return an error");

        assert_eq!(
            ContractError::InvalidSecurityType(security.to_string()).to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_must_be_owner() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked("invalid");
        let assets = vec![Addr::unchecked("asset")];
        let security = Security::new(TAG1);

        setup::mock_contract(deps.as_mut());

        let error =
            handle(deps.as_mut(), sender, &assets, &security).expect_err("should return an error");

        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }
}
