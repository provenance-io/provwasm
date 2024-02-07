use cosmwasm_std::{to_binary, Addr, Deps};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryAddressResponse},
    storage,
};

/// Performs the logic for the QueryAddress message and obtains all tags for an address.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `asset_addr` - The address to query tags for.
///
/// # Examples
/// ```
/// let res = handle(deps, addr)?;
/// ```

pub fn handle(deps: Deps, asset_addr: Addr) -> ProvQueryResponse {
    let tag = storage::tag::get_asset_tag(deps.storage, &asset_addr)?;
    let res = QueryAddressResponse { tag: tag };
    Ok(to_binary(&res)?)
}
