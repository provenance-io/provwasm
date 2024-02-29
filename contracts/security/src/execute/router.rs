use cosmwasm_std::{DepsMut, Env, MessageInfo};

use crate::core::{aliases::ProvTxResponse, msg::ExecuteMsg};

use super::{
    add_security_types, change_owner, remove_security, remove_security_types, set_security,
    set_security_multiple,
};

/// Routes the execute message to the appropriate handler based on the message's variant.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `info` - Contains the message signer and any sent funds.
/// * `msg` - The execute variant being ran by the user.
///
/// # Examples
/// ```
/// let msg = ExecuteMsg::ChangeOwner {new_owner: Addr::unchecked("new_owner")};
/// let res = route(deps, env, info, msg)?;
/// ```
pub fn route(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::ChangeOwner { new_owner } => change_owner::handle(deps, info.sender, new_owner),
        ExecuteMsg::SetSecurity {
            asset_addr,
            security,
        } => set_security::handle(deps, info.sender, asset_addr, &security),
        ExecuteMsg::SetSecurityMultiple { assets, security } => {
            set_security_multiple::handle(deps, info.sender, &assets, &security)
        }
        ExecuteMsg::RemoveSecurity { asset_addr } => {
            remove_security::handle(deps, info.sender, asset_addr)
        }
        ExecuteMsg::AddSecurityTypes { security_types } => {
            add_security_types::handle(deps, info.sender, security_types.as_slice())
        }
        ExecuteMsg::RemoveSecurityTypes { security_types } => {
            remove_security_types::handle(deps, info.sender, security_types.as_slice())
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::Security,
        storage,
        testing::{
            constants::{OWNER, TAG1},
            msg::{
                mock_add_tag_types_msg, mock_change_owner_msg, mock_remove_tag_msg,
                mock_remove_tag_types_msg, mock_set_security_multiple_msg, mock_set_tag_msg,
            },
            setup::{self, mock_info, mock_markers},
        },
        util::action::ActionType,
    };

    use super::route;

    #[test]
    fn test_change_owner_route() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info(false, OWNER);
        let msg = mock_change_owner_msg();

        setup::mock_contract(deps.as_mut());

        let res = route(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::ChangeOwner {})],
            res.attributes
        );
    }

    #[test]
    fn test_set_security_route() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info(false, OWNER);
        let asset_addr = Addr::unchecked("marker");
        mock_markers(&mut deps);
        let msg = mock_set_tag_msg(&asset_addr);

        setup::mock_contract(deps.as_mut());

        let res = route(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::SetSecurity {})],
            res.attributes
        );
    }

    #[test]
    fn test_set_security_multiple_route() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info(false, OWNER);
        let assets = vec![Addr::unchecked("marker")];
        mock_markers(&mut deps);
        let msg = mock_set_security_multiple_msg(&assets);

        setup::mock_contract(deps.as_mut());

        let res = route(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::SetSecurityMultiple {})],
            res.attributes
        );
    }

    #[test]
    fn test_remove_security_route() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info(false, OWNER);
        let asset_addr = Addr::unchecked("marker");
        mock_markers(&mut deps);
        let msg = mock_remove_tag_msg(&asset_addr);

        setup::mock_contract(deps.as_mut());
        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &Security::new(TAG1))
            .expect("should set security");

        let res = route(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::RemoveSecurity {})],
            res.attributes
        );
    }

    #[test]
    fn test_add_security_types_route() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info(false, OWNER);
        let msg = mock_add_tag_types_msg();

        setup::mock_contract(deps.as_mut());

        let res = route(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::AddSecurityTypes {})],
            res.attributes
        );
    }

    #[test]
    fn test_remove_security_types_route() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info(false, OWNER);
        let msg = mock_remove_tag_types_msg();

        setup::mock_contract(deps.as_mut());

        let res = route(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::RemoveSecurityTypes {})],
            res.attributes
        );
    }
}
