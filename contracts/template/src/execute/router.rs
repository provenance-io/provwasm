use cosmwasm_std::{Env, MessageInfo};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::ExecuteMsg,
};

use super::change_owner;

pub fn route(deps: ProvDepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::ChangeOwner { new_owner } => change_owner::handle(deps, info.sender, new_owner),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Attribute};
    use provwasm_mocks::mock_dependencies;

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
        let mut deps = mock_dependencies(&[]);
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
