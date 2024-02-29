use cosmwasm_std::{to_json_binary, Addr, Deps};

use crate::{
    core::{
        aliases::ProvQueryResponse,
        msg::{Paginate, QuerySecurityResponse, Security},
    },
    storage,
};

/// Performs the logic for the QuerySecurity messasge and obtains all addresses that contain the security.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `security` - The security to lookup addresses by.
/// * `paginate` - A struct containing additional optional args for pagination.
///
/// # Examples
/// ```
/// let res = handle(deps, &Security::new("tag1"), Paginate{limit: None, start_after: None})?;
/// ```

pub fn handle(deps: Deps, security: &Security, paginate: Paginate<Addr>) -> ProvQueryResponse {
    let assets = storage::asset::with_security(deps.storage, security, paginate)?;
    let res = QuerySecurityResponse { assets };
    Ok(to_json_binary(&res)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, Addr};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::{Paginate, QuerySecurityResponse, Security},
        query::query_security::handle,
        storage,
        testing::{
            constants::{TAG1, TAG2},
            setup,
        },
    };

    #[test]
    fn test_invalid_security() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let expected: Vec<Addr> = vec![];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let bin = handle(deps.as_ref(), &Security::new("tag3"), paginate)
            .expect("should not return an error");

        let response: QuerySecurityResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.assets);
    }

    #[test]
    fn test_single_security_single_asset() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let expected: Vec<Addr> = vec![asset_addr.clone()];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &Security::new(TAG1))
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), &Security::new(TAG1), paginate)
            .expect("should not return an error");

        let response: QuerySecurityResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.assets);
    }

    #[test]
    fn test_multi_security_single_asset() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let expected: Vec<Addr> = vec![asset_addr.clone()];
        let expected2: Vec<Addr> = vec![asset_addr2.clone()];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };

        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &Security::new(TAG1))
            .expect("should not return an error");
        storage::asset::set_security(deps.as_mut().storage, &asset_addr2, &Security::new(TAG2))
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), &Security::new(TAG1), paginate.clone())
            .expect("should not return an error");
        let response: QuerySecurityResponse =
            from_json(&bin).expect("should return correct response for first security");
        assert_eq!(expected, response.assets);

        let bin = handle(deps.as_ref(), &Security::new(TAG2), paginate)
            .expect("should not return an error");
        let response: QuerySecurityResponse =
            from_json(&bin).expect("should return correct response for second security");
        assert_eq!(expected2, response.assets);
    }

    #[test]
    fn test_single_security_multi_asset() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let expected: Vec<Addr> = vec![asset_addr.clone(), asset_addr2.clone()];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };

        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &Security::new(TAG1))
            .expect("should not return an error");
        storage::asset::set_security(deps.as_mut().storage, &asset_addr2, &Security::new(TAG1))
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), &Security::new(TAG1), paginate)
            .expect("should not return an error");
        let response: QuerySecurityResponse =
            from_json(&bin).expect("should return correct response for security");
        assert_eq!(expected, response.assets);
    }

    #[test]
    fn test_multi_security_multi_asset() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let asset_addr3 = Addr::unchecked("test3");
        let asset_addr4 = Addr::unchecked("test4");
        let expected: Vec<Addr> = vec![asset_addr.clone(), asset_addr3.clone()];
        let expected2: Vec<Addr> = vec![asset_addr2.clone(), asset_addr4.clone()];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };

        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &Security::new(TAG1))
            .expect("should not return an error");
        storage::asset::set_security(deps.as_mut().storage, &asset_addr2, &Security::new(TAG2))
            .expect("should not return an error");
        storage::asset::set_security(deps.as_mut().storage, &asset_addr3, &Security::new(TAG1))
            .expect("should not return an error");
        storage::asset::set_security(deps.as_mut().storage, &asset_addr4, &Security::new(TAG2))
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), &Security::new(TAG1), paginate.clone())
            .expect("should not return an error");
        let response: QuerySecurityResponse =
            from_json(&bin).expect("should return correct response for first tag");
        assert_eq!(expected, response.assets);

        let bin = handle(deps.as_ref(), &Security::new(TAG2), paginate)
            .expect("should not return an error");
        let response: QuerySecurityResponse =
            from_json(&bin).expect("should return correct response for second tag");
        assert_eq!(expected2, response.assets);
    }
}
