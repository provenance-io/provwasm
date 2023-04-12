use cosmwasm_std::Env;

use crate::core::{
    aliases::{ProvDeps, ProvQueryResponse},
    msg::QueryMsg,
};

use super::{query_owner, query_version};

pub fn route(deps: ProvDeps, _env: Env, msg: QueryMsg) -> ProvQueryResponse {
    match msg {
        QueryMsg::QueryOwner {} => query_owner::handle(deps),
        QueryMsg::QueryVersion {} => query_version::handle(deps),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, testing::mock_env, Addr, Coin};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::{
            constants::{CONTRACT_NAME, CONTRACT_VERSION},
            msg::{QueryMsg, QueryOwnerResponse, QueryVersionResponse},
        },
        testing::{
            constants::{OWNER, TEST_DENOM},
            setup,
        },
    };

    use super::route;

    #[test]
    fn test_query_owner_has_correct_response() {
        let mut deps = mock_dependencies(&[Coin::new(1000000000, TEST_DENOM)]);
        let env = mock_env();
        let msg = QueryMsg::QueryOwner {};
        setup::mock_contract(deps.as_mut());
        let bin = route(deps.as_ref(), env, msg).unwrap();
        let response: QueryOwnerResponse = from_binary(&bin).unwrap();
        assert_eq!(Addr::unchecked(OWNER), response.owner);
    }

    #[test]
    fn test_query_version_has_correct_response() {
        let mut deps = mock_dependencies(&[Coin::new(1000000000, TEST_DENOM)]);
        let env = mock_env();
        let msg = QueryMsg::QueryVersion {};
        setup::mock_contract(deps.as_mut());
        let bin = route(deps.as_ref(), env, msg).unwrap();
        let response: QueryVersionResponse = from_binary(&bin).unwrap();
        assert_eq!(CONTRACT_NAME, response.contract_version.contract);
        assert_eq!(CONTRACT_VERSION, response.contract_version.version);
    }
}
