use cosmwasm_std::{to_binary, Deps};

use crate::core::{aliases::ProvQueryResponse, msg::QueryTagResponse};

/// Performs the logic for the QueryTag messasge and obtains all addresses that contain the tag.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `tag` - The tag to search for
///
/// # Examples
/// ```
/// let res = handle(deps, tag)?;
/// ```

pub fn handle(_: Deps, _: String) -> ProvQueryResponse {
    let res = QueryTagResponse { assets: vec![] };
    Ok(to_binary(&res)?)
}
