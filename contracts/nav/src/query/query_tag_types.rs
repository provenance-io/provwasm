use cosmwasm_std::{to_json_binary, Deps};

use crate::{
    core::{aliases::ProvQueryResponse, msg::QueryTagTypesResponse},
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

pub fn handle(deps: Deps) -> ProvQueryResponse {
    let tags = storage::tag::get_types(deps.storage)?;
    let res = QueryTagTypesResponse { tags };
    Ok(to_json_binary(&res)?)
}
