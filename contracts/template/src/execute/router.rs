use cosmwasm_std::{DepsMut, Env, MessageInfo};

use crate::core::{aliases::ProvTxResponse, msg::ExecuteMsg};

use super::change_owner;

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
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Attribute};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        testing::{
            constants::OWNER,
            msg::mock_change_owner_msg,
            setup::{self, mock_info},
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
}
