use cosmwasm_std::{Addr, DepsMut, Response};
use provwasm_std::types::provenance::{marker::v1::MarkerQuerier, metadata::v1::MetadataQuerier};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError},
    events::set_tag::SetTagEvent,
    storage,
    util::action::{Action, ActionType},
};

/// Performs the execute logic for the SetTag variant of ExecuteMsg.
///
/// If the sender is the owner of the contract, then the contract will update the tag
/// for the asset. The updated asset must be a marker or a scope, and when an empty tag is provided
/// the asset's tag will be removed.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `asset_addr` - The address of the asset to set the tag for.
/// * `tag` - The label to set as the asset's tag.
///
/// # Examples
/// ```
/// let msg = ExecuteMsg::SetTag {asset_addr: Addr::unchecked("addr"), tag: "tag"};
/// let res = handle(deps, env, info.sender, msg.asset_addr, &msg.tag)?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, asset_addr: Addr, tag: &str) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    if !is_marker(&deps, &asset_addr)? && !is_scope(&deps, &asset_addr)? {
        return Err(ContractError::AssetDoesNotExist(asset_addr.to_string()));
    }

    if tag.is_empty() {
        storage::asset::remove_tag(deps.storage, &asset_addr);
    } else {
        if !storage::tag::has_type(deps.storage, tag) {
            return Err(ContractError::InvalidTagType(tag.to_string()));
        }

        storage::asset::set_tag(deps.storage, &asset_addr, tag)?;
    }

    Ok(Response::default()
        .set_action(ActionType::SetTag {})
        .add_event(SetTagEvent::new(&asset_addr, tag).into()))
}

fn is_marker(deps: &DepsMut, asset_addr: &Addr) -> Result<bool, ContractError> {
    let marker = MarkerQuerier::new(&deps.querier);
    let response = marker.marker(asset_addr.to_string())?;
    Ok(response.marker.is_some())
}

fn is_scope(deps: &DepsMut, asset_addr: &Addr) -> Result<bool, ContractError> {
    let metadata = MetadataQuerier::new(&deps.querier);
    let response = metadata.scope(
        asset_addr.to_string(),
        "".to_string(),
        "".to_string(),
        false,
        false,
    )?;
    Ok(response.scope.is_some())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::error::ContractError,
        events::set_tag::SetTagEvent,
        storage,
        testing::{
            constants::{OWNER, TAG1},
            setup,
        },
        util::action::ActionType,
    };

    use super::handle;

    #[test]
    fn test_adds_tag_for_valid_marker() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("marker");
        let tag = TAG1;
        let expected_events: Vec<Event> = vec![SetTagEvent::new(&asset_addr, tag).into()];
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let res = handle(deps.as_mut(), sender, asset_addr.clone(), tag)
            .expect("should not return an error");
        let stored_tag =
            storage::asset::get_tag(&deps.storage, &asset_addr).expect("should have tag for asset");

        assert_eq!(tag.to_string(), stored_tag);
        assert_eq!(vec![Attribute::from(ActionType::SetTag {})], res.attributes);
        assert_eq!(expected_events, res.events);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_adds_tag_for_valid_scope() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("scope");
        let tag = TAG1;
        let expected_events: Vec<Event> = vec![SetTagEvent::new(&asset_addr, tag).into()];
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let res = handle(deps.as_mut(), sender, asset_addr.clone(), tag)
            .expect("should not return an error");
        let stored_tag =
            storage::asset::get_tag(&deps.storage, &asset_addr).expect("should have tag for asset");

        assert_eq!(tag.to_string(), stored_tag);
        assert_eq!(vec![Attribute::from(ActionType::SetTag {})], res.attributes);
        assert_eq!(expected_events, res.events);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_removes_tag_for_valid_marker() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("marker");
        let tag = "";
        let expected_events: Vec<Event> = vec![SetTagEvent::new(&asset_addr, tag).into()];
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        storage::asset::set_tag(deps.as_mut().storage, &asset_addr, TAG1).expect("should set tag");
        let res = handle(deps.as_mut(), sender, asset_addr.clone(), tag)
            .expect("should not return an error");
        let found = storage::asset::has_tag(&deps.storage, tag);

        assert_eq!(false, found);
        assert_eq!(vec![Attribute::from(ActionType::SetTag {})], res.attributes);
        assert_eq!(expected_events, res.events);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_removes_tag_for_valid_scope() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("scope");
        let tag = "";
        let expected_events: Vec<Event> = vec![SetTagEvent::new(&asset_addr, tag).into()];
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        storage::asset::set_tag(deps.as_mut().storage, &asset_addr, TAG1).expect("should set tag");
        let res = handle(deps.as_mut(), sender, asset_addr.clone(), tag)
            .expect("should not return an error");
        let found = storage::asset::has_tag(&deps.storage, tag);

        assert_eq!(false, found);
        assert_eq!(vec![Attribute::from(ActionType::SetTag {})], res.attributes);
        assert_eq!(expected_events, res.events);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn test_fails_for_invalid_addr() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("invalid");
        let tag = "";
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let err = handle(deps.as_mut(), sender, asset_addr.clone(), tag)
            .expect_err("should not return an error");

        assert_eq!(
            ContractError::AssetDoesNotExist(asset_addr.to_string()).to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_fails_for_invalid_tag() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("marker");
        let tag = "invalid";
        setup::mock_scopes(&mut deps);
        setup::mock_markers(&mut deps);

        setup::mock_contract(deps.as_mut());

        let err = handle(deps.as_mut(), sender, asset_addr.clone(), tag)
            .expect_err("should not return an error");

        assert_eq!(
            ContractError::InvalidTagType(tag.to_string()).to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_must_be_owner() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked("invalid");
        let asset_addr = Addr::unchecked("asset");
        let tag = TAG1;

        setup::mock_contract(deps.as_mut());

        let error =
            handle(deps.as_mut(), sender, asset_addr, tag).expect_err("should return an error");

        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }
}
