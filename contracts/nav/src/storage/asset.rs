use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Map;

use crate::core::{
    aliases::AssetTag,
    constants::{ASSET_TAG_KEY, TAG_TO_ASSET_KEY},
    error::ContractError,
};

pub const ASSET_TO_TAG: Map<&Addr, AssetTag> = Map::new(ASSET_TAG_KEY);
pub const TAG_TO_ASSET: Map<(AssetTag, &Addr), ()> = Map::new(TAG_TO_ASSET_KEY);

/// Attempts to get the asset's tag that is stored within the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `asset_addr` - The address of the asset to retrieve the tag of.
///
/// # Examples
/// ```
/// let addr = Addr::unchecked("addr1");
/// get_tag(deps.storage, &addr)?;
/// `
pub fn get_tag(storage: &dyn Storage, asset_addr: &Addr) -> Result<String, ContractError> {
    return Ok(ASSET_TO_TAG.load(storage, asset_addr)?);
}

/// Attempts to get all assets that have the specified tag.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `tag` - The tag to do a lookup by.
///
/// # Examples
/// ```
/// with_tag(deps.storage, "tag")?;
/// `
pub fn with_tag(storage: &dyn Storage, tag: &str) -> Result<Vec<Addr>, ContractError> {
    let assets: Result<Vec<Addr>, ContractError> = TAG_TO_ASSET
        .prefix(tag.to_string())
        .keys(storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|result| result.map_err(|err| ContractError::Std(err)))
        .collect();
    return assets;
}

/// Attempts to check if any assets has the supplied tag.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `tag` - The tag to look for.
///
/// # Examples
/// ```
/// has_tag(deps.storage, "tag")?;
/// `
pub fn has_tag(storage: &dyn Storage, tag: &str) -> bool {
    let tag_is_used = !TAG_TO_ASSET.prefix(tag.to_string()).is_empty(storage);
    return tag_is_used;
}

/// Attempts to set the asset's tag in the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `asset_addr` - The address of the asset to tag.
/// * `tag` - The label to add to the asset.
///
/// # Examples
/// ```
/// let addr = Addr::unchecked("addr1");
/// set_tag(deps.as_mut().storage, &addr, "tag")?;
/// `
pub fn set_tag(
    storage: &mut dyn Storage,
    asset_addr: &Addr,
    tag: &str,
) -> Result<(), ContractError> {
    Ok(ASSET_TO_TAG.save(storage, asset_addr, &tag.to_string())?)
}

/// Removes the asset's tag from the contract's storage.
///
/// # Arguments
///
/// * `storage` - A reference to the Deps' Storage object.
/// * `asset_addr` - The address of the asset to tag.
///
/// # Examples
/// ```
/// let addr = Addr::unchecked("addr1");
/// remove_tag(deps.as_mut().storage, &addr);
/// `
pub fn remove_tag(storage: &mut dyn Storage, asset_addr: &Addr) {
    ASSET_TO_TAG.remove(storage, asset_addr);
}
