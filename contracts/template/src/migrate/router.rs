use cosmwasm_std::Env;

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::MigrateMsg,
};

use super::default;

pub fn route(deps: &ProvDepsMut, env: Env, msg: MigrateMsg) -> ProvTxResponse {
    match msg {
        MigrateMsg::Default {} => default::handle(deps, env, msg),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Attribute};
    use provwasm_mocks::mock_dependencies;

    use crate::{core::msg::MigrateMsg, util::action::ActionType};

    use super::route;

    #[test]
    fn test_default_route() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let msg = MigrateMsg::Default {};
        let res = route(&deps.as_mut(), env, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::Migrate {})],
            res.attributes
        );
        assert_eq!(0, res.events.len());
        assert_eq!(0, res.messages.len());
    }
}
