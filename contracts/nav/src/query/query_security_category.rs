use cosmwasm_std::{to_json_binary, Addr, Deps};

use crate::{
    core::{
        aliases::ProvQueryResponse,
        msg::{Paginate, QuerySecurityCategoryResponse},
    },
    storage,
};

/// Performs the logic for the QuerySecurityCategory messasge and obtains all addresses that contain the security category.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `security` - The security to lookup addresses by.
/// * `paginate` - A struct containing additional optional args for pagination.
///
/// # Examples
/// ```
/// let res = handle(deps, &Security::new("tag1"), Paginate{limit: None, start_after: None})?;
/// ```

pub fn handle(deps: Deps, category: &str, paginate: Paginate<(String, Addr)>) -> ProvQueryResponse {
    let assets = storage::asset::with_security_category(deps.storage, category, paginate)?;
    let res = QuerySecurityCategoryResponse { assets };
    Ok(to_json_binary(&res)?)
}
