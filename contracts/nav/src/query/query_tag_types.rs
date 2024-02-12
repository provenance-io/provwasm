use cosmwasm_std::{to_json_binary, Deps};

use crate::{
    core::{
        aliases::{AssetTag, ProvQueryResponse},
        msg::{Paginate, QueryTagTypesResponse},
    },
    storage,
};

/// Performs the logic for the QueryTagTypes messasge and obtains all accepted tag types.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
///
/// # Examples
/// ```
/// let res = handle(deps)?;
/// ```

pub fn handle(deps: Deps, paginate: Paginate<AssetTag>) -> ProvQueryResponse {
    let tags = storage::tag::get_types(deps.storage, paginate)?;
    let res = QueryTagTypesResponse { tags };
    Ok(to_json_binary(&res)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::from_json;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::QueryTagTypesResponse,
        query::query_tag_types::handle,
        storage::{self},
        testing::{
            constants::{TAG1, TAG2},
            setup,
        },
    };

    #[test]
    fn test_contract_with_tag_types() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let expected = vec![TAG1.to_string(), TAG2.to_string()];

        let bin = handle(deps.as_ref()).expect("should not return an error");

        let response: QueryTagTypesResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.tags);
    }

    #[test]
    fn test_contract_with_single_tag_type() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let expected = vec![TAG2.to_string()];

        storage::tag::remove_type(deps.as_mut().storage, TAG1);

        let bin = handle(deps.as_ref()).expect("should not return an error");

        let response: QueryTagTypesResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.tags);
    }

    #[test]
    fn test_contract_with_no_tag_types() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let expected: Vec<String> = vec![];

        storage::tag::remove_type(deps.as_mut().storage, TAG1);
        storage::tag::remove_type(deps.as_mut().storage, TAG2);

        let bin = handle(deps.as_ref()).expect("should not return an error");

        let response: QueryTagTypesResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.tags);
    }
}
