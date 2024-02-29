use cosmwasm_std::{to_json_binary, Deps};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryOwnerResponse},
    storage,
};

/// Performs the logic for the QueryOwner message and obtains the contract's owner.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
///
/// # Examples
/// ```
/// let res = handle(deps)?;
/// ```
pub fn handle(deps: Deps) -> ProvQueryResponse {
    let res = QueryOwnerResponse {
        owner: storage::state::get_owner(deps.storage)?,
    };
    Ok(to_json_binary(&res)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, Addr};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::msg::QueryOwnerResponse,
        testing::{constants::OWNER, setup},
    };

    use super::handle;

    #[test]
    fn test_query_owner_has_correct_response() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let bin = handle(deps.as_ref()).unwrap();
        let response: QueryOwnerResponse = from_json(&bin).unwrap();
        assert_eq!(Addr::unchecked(OWNER), response.owner);
    }
}
