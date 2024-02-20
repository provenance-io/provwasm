use cosmwasm_std::{Deps, Env};

use crate::core::{aliases::ProvQueryResponse, msg::QueryMsg};

use super::{
    query_address, query_owner, query_security, query_security_category, query_security_types,
    query_version,
};

/// Routes the query message to the appropriate handler based on the message's variant.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `msg` - The query being ran by the user.
///
/// # Examples
/// ```
/// let msg = QueryMsg::QueryVersion {};
/// let res = route(deps, env, msg)?;
/// ```
pub fn route(deps: Deps, _env: Env, msg: QueryMsg) -> ProvQueryResponse {
    match msg {
        QueryMsg::QueryOwner {} => query_owner::handle(deps),
        QueryMsg::QueryVersion {} => query_version::handle(deps),
        QueryMsg::QueryAddress { asset_addr } => query_address::handle(deps, asset_addr),
        QueryMsg::QuerySecurity { security, paginate } => {
            query_security::handle(deps, &security, paginate)
        }
        QueryMsg::QuerySecurityTypes { paginate } => query_security_types::handle(deps, paginate),
        QueryMsg::QuerySecurityCategory { category, paginate } => {
            query_security_category::handle(deps, &category, paginate)
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, testing::mock_env, Addr};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{
            constants::{CONTRACT_NAME, CONTRACT_VERSION},
            msg::{
                Paginate, QueryAddressResponse, QueryMsg, QueryOwnerResponse,
                QuerySecurityResponse, QuerySecurityTypesResponse, QueryVersionResponse, Security,
            },
        },
        storage,
        testing::{
            constants::{OWNER, TAG1, TAG2},
            setup,
        },
    };

    use super::route;

    #[test]
    fn test_query_owner_has_correct_response() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let msg = QueryMsg::QueryOwner {};
        setup::mock_contract(deps.as_mut());
        let bin = route(deps.as_ref(), env, msg).unwrap();
        let response: QueryOwnerResponse = from_json(&bin).unwrap();
        assert_eq!(Addr::unchecked(OWNER), response.owner);
    }

    #[test]
    fn test_query_version_has_correct_response() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let msg = QueryMsg::QueryVersion {};
        setup::mock_contract(deps.as_mut());
        let bin = route(deps.as_ref(), env, msg).unwrap();
        let response: QueryVersionResponse = from_json(&bin).unwrap();
        assert_eq!(CONTRACT_NAME, response.contract_version.contract);
        assert_eq!(CONTRACT_VERSION, response.contract_version.version);
    }

    #[test]
    fn test_query_address_has_correct_response() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let asset_addr = Addr::unchecked("marker");
        let security = Security::new("category");
        let msg = QueryMsg::QueryAddress {
            asset_addr: asset_addr.clone(),
        };
        setup::mock_contract(deps.as_mut());
        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &security)
            .expect("should set the security for asset");
        let bin = route(deps.as_ref(), env, msg).unwrap();
        let response: QueryAddressResponse = from_json(&bin).unwrap();
        assert_eq!(security, response.security);
    }

    #[test]
    fn test_query_security_has_correct_response() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let asset_addr = Addr::unchecked("marker");
        let security = Security::new("category");
        let msg = QueryMsg::QuerySecurity {
            security: security.clone(),
            paginate: Paginate {
                limit: None,
                start_after: None,
            },
        };
        setup::mock_contract(deps.as_mut());
        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &security)
            .expect("should set the security for asset");
        let bin = route(deps.as_ref(), env, msg).unwrap();
        let response: QuerySecurityResponse = from_json(&bin).unwrap();
        assert_eq!(vec![asset_addr], response.assets);
    }

    #[test]
    fn test_security_types_has_correct_response() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let msg = QueryMsg::QuerySecurityTypes {
            paginate: Paginate {
                limit: None,
                start_after: None,
            },
        };
        setup::mock_contract(deps.as_mut());
        let bin = route(deps.as_ref(), env, msg).unwrap();
        let response: QuerySecurityTypesResponse = from_json(&bin).unwrap();
        let expected = vec![Security::new(TAG1), Security::new(TAG2)];
        assert_eq!(expected, response.securities);
    }
}
