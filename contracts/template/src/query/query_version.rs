use cosmwasm_std::to_binary;
use cw2::get_contract_version;

use crate::core::{
    aliases::{ProvDeps, ProvQueryResponse},
    msg::QueryVersionResponse,
};

/// Performs the logic for the QueryVersion message and obtains the contract version.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
///
/// # Examples
/// ```
/// let res = handle(deps)?;
/// ```
pub fn handle(deps: ProvDeps) -> ProvQueryResponse {
    let res = QueryVersionResponse {
        contract_version: get_contract_version(deps.storage)?,
    };
    Ok(to_binary(&res)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, Coin};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::{
            constants::{CONTRACT_NAME, CONTRACT_VERSION},
            msg::QueryVersionResponse,
        },
        testing::{constants::TEST_DENOM, setup},
    };

    use super::handle;

    #[test]
    fn test_query_version_has_correct_response() {
        let mut deps = mock_dependencies(&[Coin::new(1000000000, TEST_DENOM)]);
        setup::mock_contract(deps.as_mut());
        let bin = handle(deps.as_ref()).unwrap();
        let response: QueryVersionResponse = from_binary(&bin).unwrap();
        assert_eq!(CONTRACT_NAME, response.contract_version.contract);
        assert_eq!(CONTRACT_VERSION, response.contract_version.version);
    }
}
