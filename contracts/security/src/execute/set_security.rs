use cosmwasm_std::{Addr, DepsMut, Response};
use provwasm_std::types::provenance::{marker::v1::MarkerQuerier, metadata::v1::MetadataQuerier};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError, msg::Security},
    events::set_security::SetSecurityEvent,
    storage,
    util::action::{Action, ActionType},
};

/// Performs the execute logic for the SetSecurity variant of ExecuteMsg.
///
/// If the sender is the owner of the contract, then the contract will update the security
/// for the asset. The updated asset must be a marker or a scope.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `asset_addr` - The address of the asset to set the security for.
/// * `security` - The security to link to the asset.
///
/// # Examples
/// ```
/// let msg = ExecuteMsg::SetSecurity {asset_addr: Addr::unchecked("addr"), security: Security::new("tag")};
/// let res = handle(deps, env, info.sender, msg.asset_addr, &msg.security)?;
/// ```
pub fn handle(
    mut deps: DepsMut,
    sender: Addr,
    asset_addr: Addr,
    security: &Security,
) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    set_security(&mut deps, &asset_addr, security)?;

    Ok(Response::default()
        .set_action(ActionType::SetSecurity {})
        .add_event(SetSecurityEvent::new(&asset_addr, security).into()))
}

pub fn set_security(
    deps: &mut DepsMut,
    asset_addr: &Addr,
    security: &Security,
) -> Result<(), ContractError> {
    if !is_marker(deps, asset_addr)? && !is_scope(deps, asset_addr)? {
        return Err(ContractError::AssetDoesNotExist(asset_addr.to_string()));
    }

    if !storage::security::has_type(deps.storage, security) {
        return Err(ContractError::InvalidSecurityType(security.to_string()));
    }

    storage::asset::set_security(deps.storage, asset_addr, security)
}

fn is_marker(deps: &DepsMut, asset_addr: &Addr) -> Result<bool, ContractError> {
    let marker = MarkerQuerier::new(&deps.querier);
    let response = marker.marker(asset_addr.to_string());
    Ok(response.is_ok_and(|res| res.marker.is_some()))
}

fn is_scope(deps: &DepsMut, asset_addr: &Addr) -> Result<bool, ContractError> {
    let metadata = MetadataQuerier::new(&deps.querier);
    let response = metadata.scope(
        asset_addr.to_string(),
        "".to_string(),
        "".to_string(),
        false,
        false,
    );
    Ok(response.is_ok_and(|res| res.scope.is_some()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{error::ContractError, msg::Security},
        events::set_security::SetSecurityEvent,
        storage,
        testing::{
            constants::{OWNER, TAG1},
            setup,
        },
        util::action::ActionType,
    };

    use super::handle;

    #[test]
    fn test_adds_security_for_valid_marker() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("marker");
        let security = Security::new(TAG1);
        let expected_events: Vec<Event> =
            vec![SetSecurityEvent::new(&asset_addr, &security).into()];
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let res = handle(deps.as_mut(), sender, asset_addr.clone(), &security)
            .expect("should not return an error");
        let stored_security = storage::asset::get_security(&deps.storage, &asset_addr)
            .expect("should have security for asset");

        assert_eq!(security, stored_security);
        assert_eq!(
            vec![Attribute::from(ActionType::SetSecurity {})],
            res.attributes
        );
        assert_eq!(expected_events, res.events);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_adds_tag_for_valid_scope() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("scope");
        let security = Security::new(TAG1);
        let expected_events: Vec<Event> =
            vec![SetSecurityEvent::new(&asset_addr, &security).into()];
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let res = handle(deps.as_mut(), sender, asset_addr.clone(), &security)
            .expect("should not return an error");
        let stored_security = storage::asset::get_security(&deps.storage, &asset_addr)
            .expect("should have security for asset");

        assert_eq!(security, stored_security);
        assert_eq!(
            vec![Attribute::from(ActionType::SetSecurity {})],
            res.attributes
        );
        assert_eq!(expected_events, res.events);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_fails_for_invalid_addr() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("invalid");
        let security: Security = Security::new("");
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let err = handle(deps.as_mut(), sender, asset_addr.clone(), &security)
            .expect_err("should not return an error");

        assert_eq!(
            ContractError::AssetDoesNotExist(asset_addr.to_string()).to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_fails_for_invalid_security() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("marker");
        let security: Security = Security::new("invalid");
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let err = handle(deps.as_mut(), sender, asset_addr.clone(), &security)
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
        let asset_addr = Addr::unchecked("asset");
        let security = Security::new(TAG1);

        setup::mock_contract(deps.as_mut());

        let error = handle(deps.as_mut(), sender, asset_addr, &security)
            .expect_err("should return an error");

        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }
}
