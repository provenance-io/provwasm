use cosmwasm_std::{Env, Response};

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        msg::MigrateMsg,
    },
    util::action::{Action, ActionType},
};

pub fn handle(_deps: &ProvDepsMut, _env: Env, _msg: MigrateMsg) -> ProvTxResponse {
    Ok(Response::new().set_action(ActionType::Migrate {}))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Attribute};
    use provwasm_mocks::mock_dependencies;

    use crate::{core::msg::MigrateMsg, util::action::ActionType};

    use super::handle;

    #[test]
    fn test_handle() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let msg = MigrateMsg::Default {};
        let res = handle(&deps.as_mut(), env, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::Migrate {})],
            res.attributes
        );
        assert_eq!(0, res.events.len());
        assert_eq!(0, res.messages.len());
    }
}
