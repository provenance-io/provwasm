use cosmwasm_std::Storage;
use cw_storage_plus::Map;

use crate::core::{constants::ASSET_TYPE_KEY, error::ContractError};

pub const TAG_TYPES: Map<&str, ()> = Map::new(ASSET_TYPE_KEY);

/// Attempts to add a tag to the list of acceptable types in the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `tag` - The label to add to the list of acceptable types.
///
/// # Examples
/// ```
/// add_type(deps.as_mut().storage, "tag")?;
/// `
pub fn add_type(storage: &mut dyn Storage, tag: &str) -> Result<(), ContractError> {
    Ok(TAG_TYPES.save(storage, tag, &())?)
}

/// Removes the tag type from the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `tag` - The label to remove from the list of acceptable types.
///
/// # Examples
/// ```
/// remove_type(deps.as_mut().storage, "tag");
/// `
pub fn remove_type(storage: &mut dyn Storage, tag: &str) {
    TAG_TYPES.remove(storage, tag);
}

/// Checks if the tag is in the list of the contract's accepted tag types.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `tag` - The label to check if it's in the list of acceptable types.
///
/// # Examples
/// ```
/// has_type(deps.storage, "tag");
/// `
pub fn has_type(storage: &dyn Storage, tag: &str) -> bool {
    return TAG_TYPES.has(storage, tag);
}

/// Obtains all the accepted tag types from the contract's store.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
///
/// # Examples
/// ```
/// get_types(deps.storage);
/// `
pub fn get_types(storage: &dyn Storage) -> Result<Vec<String>, ContractError> {
    let keys: Result<Vec<String>, ContractError> = TAG_TYPES
        .keys(storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|result| result.map_err(|err| ContractError::Std(err)))
        .collect();
    keys
}
