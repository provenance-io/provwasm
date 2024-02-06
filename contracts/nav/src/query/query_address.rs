use cosmwasm_std::{to_binary, Addr, Deps};

use crate::core::{aliases::ProvQueryResponse, msg::QueryAddressResponse};

/// Performs the logic for the QueryAddress message and obtains all tags for an address.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `addr` - The address to query tags for.
///
/// # Examples
/// ```
/// let res = handle(deps, addr)?;
/// ```

pub fn handle(_: Deps, _: Addr) -> ProvQueryResponse {
    let res = QueryAddressResponse {
        tag: "".to_string(),
    };
    Ok(to_binary(&res)?)
}
