use cosmwasm_std::{to_json_binary, Deps};

use crate::{
    core::{
        aliases::ProvQueryResponse,
        msg::{Paginate, QuerySecurityTypesResponse, Security},
    },
    storage,
};

/// Performs the logic for the QuerySecurityTypes messasge and obtains all accepted security types.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `paginate` - A struct containing additional optional args for pagination.
///
/// # Examples
/// ```
/// let res = handle(deps, Paginate{limit: None, start_after: None})?;
/// ```

pub fn handle(deps: Deps, paginate: Paginate<Security>) -> ProvQueryResponse {
    let securities = storage::security::get_types(deps.storage, paginate)?;
    let res = QuerySecurityTypesResponse { securities };
    Ok(to_json_binary(&res)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::from_json;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::{Paginate, QuerySecurityTypesResponse, Security},
        query::query_security_types::handle,
        storage,
        testing::{
            constants::{TAG1, TAG2},
            setup,
        },
    };

    #[test]
    fn test_contract_with_tag_types() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let expected = vec![Security::new(TAG1), Security::new(TAG2)];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };

        let bin = handle(deps.as_ref(), paginate).expect("should not return an error");

        let response: QuerySecurityTypesResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.securities);
    }

    #[test]
    fn test_contract_with_single_tag_type() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let expected = vec![Security::new(TAG2)];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };

        storage::security::remove_type(deps.as_mut().storage, &Security::new(TAG1));

        let bin = handle(deps.as_ref(), paginate).expect("should not return an error");

        let response: QuerySecurityTypesResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.securities);
    }

    #[test]
    fn test_contract_with_no_tag_types() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let expected: Vec<Security> = vec![];
        let paginate = Paginate {
            limit: None,
            start_after: None,
        };

        storage::security::remove_type(deps.as_mut().storage, &Security::new(TAG1));
        storage::security::remove_type(deps.as_mut().storage, &Security::new(TAG2));

        let bin = handle(deps.as_ref(), paginate).expect("should not return an error");

        let response: QuerySecurityTypesResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.securities);
    }
}
