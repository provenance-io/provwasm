use cosmwasm_std::{to_json_binary, Addr, Deps};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryAddressResponse},
    storage,
};

/// Performs the logic for the QueryAddress message and obtains the security linked to an address.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `asset_addr` - The address to query the security for.
///
/// # Examples
/// ```
/// let res = handle(deps, addr)?;
/// ```

pub fn handle(deps: Deps, asset_addr: Addr) -> ProvQueryResponse {
    let security = storage::asset::get_security(deps.storage, &asset_addr)?;
    let res = QueryAddressResponse { security };
    Ok(to_json_binary(&res)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, Addr, StdError};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{
            error::ContractError,
            msg::{QueryAddressResponse, Security},
        },
        storage,
        testing::setup,
    };

    use super::handle;

    #[test]
    fn test_invalid_address() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let err = handle(deps.as_ref(), asset_addr).expect_err("should return an error");

        assert!(matches!(
            err,
            ContractError::Std(StdError::NotFound { kind: _ })
        ));
    }

    #[test]
    fn test_valid_address() {
        let mut deps = mock_provenance_dependencies();
        setup::mock_contract(deps.as_mut());
        let asset_addr = Addr::unchecked("test");
        let security = Security::new("tag");

        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &security)
            .expect("should not throw error");
        let bin = handle(deps.as_ref(), asset_addr).expect("should not return an error");

        let response: QueryAddressResponse =
            from_json(&bin).expect("should return correct response");
        assert_eq!(security, response.security);
    }
}
