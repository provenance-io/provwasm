use cosmwasm_std::{to_json_binary, Addr, Deps};

use crate::{
    core::{
        aliases::ProvQueryResponse,
        msg::{Paginate, QuerySecurityCategoryResponse},
    },
    storage,
};

/// Performs the logic for the QuerySecurityCategory messasge and obtains all addresses that contain the security category.
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

pub fn handle(deps: Deps, category: &str, paginate: Paginate<(String, Addr)>) -> ProvQueryResponse {
    let assets = storage::asset::with_security_category(deps.storage, category, paginate)?;
    let res = QuerySecurityCategoryResponse { assets };
    Ok(to_json_binary(&res)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, Addr};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::{Paginate, QuerySecurityCategoryResponse, Security},
        query::query_security_category::handle,
        storage,
        testing::{
            constants::{TAG1, TAG2},
            setup,
        },
    };

    #[test]
    fn test_invalid_category() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let expected: Vec<(String, Addr)> = vec![];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        let bin = handle(deps.as_ref(), "tag3", paginate).expect("should not return an error");

        let response: QuerySecurityCategoryResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.assets);
    }

    #[test]
    fn test_single_security_single_category() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let expected: Vec<(String, Addr)> = vec![("".to_string(), asset_addr.clone())];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &Security::new(TAG1))
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), TAG1, paginate).expect("should not return an error");

        let response: QuerySecurityCategoryResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.assets);
    }

    #[test]
    fn test_multi_security_single_category() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let expected: Vec<(String, Addr)> = vec![
            ("".to_string(), asset_addr.clone()),
            ("".to_string(), asset_addr2.clone()),
        ];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &Security::new(TAG1))
            .expect("should not return an error");
        storage::asset::set_security(deps.as_mut().storage, &asset_addr2, &Security::new(TAG1))
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), TAG1, paginate).expect("should not return an error");

        let response: QuerySecurityCategoryResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.assets);
    }

    #[test]
    fn test_multi_security_multi_category() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let expected: Vec<(String, Addr)> = vec![("".to_string(), asset_addr.clone())];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };
        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &Security::new(TAG1))
            .expect("should not return an error");
        storage::asset::set_security(deps.as_mut().storage, &asset_addr2, &Security::new(TAG2))
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), TAG1, paginate).expect("should not return an error");

        let response: QuerySecurityCategoryResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.assets);
    }
}
