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

pub fn handle(deps: Deps, tag: String) -> ProvQueryResponse {
    let assets = storage::asset::with_tag(deps.storage, &tag)?;
    let res = QueryTagResponse { assets };
    Ok(to_json_binary(&res)?)
}
