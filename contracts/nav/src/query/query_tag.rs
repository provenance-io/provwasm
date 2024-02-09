use cosmwasm_std::{to_json_binary, Deps};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryTagResponse},
    storage,
};

/// Performs the logic for the QueryTag messasge and obtains all addresses that contain the tag.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `tag` - The tag to lookup addresses by.
///
/// # Examples
/// ```
/// let res = handle(deps, tag)?;
/// ```

pub fn handle(deps: Deps, tag: &str) -> ProvQueryResponse {
    let assets = storage::asset::with_tag(deps.storage, tag)?;
    let res = QueryTagResponse { assets };
    Ok(to_json_binary(&res)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, Addr};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::QueryTagResponse,
        query::query_tag::handle,
        storage,
        testing::{
            constants::{TAG1, TAG2},
            setup,
        },
    };

    #[test]
    fn test_invalid_tag() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let expected: Vec<Addr> = vec![];

        let bin = handle(deps.as_ref(), "tag3").expect("should not return an error");

        let response: QueryTagResponse = from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.assets);
    }

    #[test]
    fn test_single_tag_single_asset() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let expected: Vec<Addr> = vec![asset_addr.clone()];

        storage::asset::set_tag(deps.as_mut().storage, &asset_addr, TAG1)
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), "tag1").expect("should not return an error");

        let response: QueryTagResponse = from_json(&bin).expect("should return correct response");
        assert_eq!(expected, response.assets);
    }

    #[test]
    fn test_multi_tag_single_asset() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let expected: Vec<Addr> = vec![asset_addr.clone()];
        let expected2: Vec<Addr> = vec![asset_addr2.clone()];

        storage::asset::set_tag(deps.as_mut().storage, &asset_addr, TAG1)
            .expect("should not return an error");
        storage::asset::set_tag(deps.as_mut().storage, &asset_addr2, TAG2)
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), "tag1").expect("should not return an error");
        let response: QueryTagResponse =
            from_json(&bin).expect("should return correct response for first tag");
        assert_eq!(expected, response.assets);

        let bin = handle(deps.as_ref(), "tag2").expect("should not return an error");
        let response: QueryTagResponse =
            from_json(&bin).expect("should return correct response for second tag");
        assert_eq!(expected2, response.assets);
    }

    #[test]
    fn test_single_tag_multi_asset() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let expected: Vec<Addr> = vec![asset_addr.clone(), asset_addr2.clone()];

        storage::asset::set_tag(deps.as_mut().storage, &asset_addr, TAG1)
            .expect("should not return an error");
        storage::asset::set_tag(deps.as_mut().storage, &asset_addr2, TAG1)
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), "tag1").expect("should not return an error");
        let response: QueryTagResponse =
            from_json(&bin).expect("should return correct response for first tag");
        assert_eq!(expected, response.assets);
    }

    #[test]
    fn test_multi_tag_multi_asset() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let asset_addr2 = Addr::unchecked("test2");
        let asset_addr3 = Addr::unchecked("test3");
        let asset_addr4 = Addr::unchecked("test4");
        let expected: Vec<Addr> = vec![asset_addr.clone(), asset_addr3.clone()];
        let expected2: Vec<Addr> = vec![asset_addr2.clone(), asset_addr4.clone()];

        storage::asset::set_tag(deps.as_mut().storage, &asset_addr, TAG1)
            .expect("should not return an error");
        storage::asset::set_tag(deps.as_mut().storage, &asset_addr2, TAG2)
            .expect("should not return an error");
        storage::asset::set_tag(deps.as_mut().storage, &asset_addr3, TAG1)
            .expect("should not return an error");
        storage::asset::set_tag(deps.as_mut().storage, &asset_addr4, TAG2)
            .expect("should not return an error");

        let bin = handle(deps.as_ref(), "tag1").expect("should not return an error");
        let response: QueryTagResponse =
            from_json(&bin).expect("should return correct response for first tag");
        assert_eq!(expected, response.assets);

        let bin = handle(deps.as_ref(), "tag2").expect("should not return an error");
        let response: QueryTagResponse =
            from_json(&bin).expect("should return correct response for second tag");
        assert_eq!(expected2, response.assets);
    }
}
