use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Map;

use crate::core::{aliases::AssetTag, constants::ASSET_TAG_KEY, error::ContractError};

pub const ASSET_TO_TAG: Map<&Addr, AssetTag> = Map::new(ASSET_TAG_KEY);

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
/// get_asset_tag(deps.storage, &addr)?;
/// `
pub fn get_asset_tag(storage: &dyn Storage, asset_addr: &Addr) -> Result<String, ContractError> {
    return Ok(ASSET_TO_TAG.load(storage, asset_addr)?);
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
/// set_asset_tag(deps.as_mut().storage, &state, &addr, "tag")?;
/// `
pub fn set_asset_tag(
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
/// remove_asset_tag(deps.as_mut().storage, &state, &addr);
/// `
pub fn remove_asset_tag(storage: &mut dyn Storage, asset_addr: &Addr) {
    ASSET_TO_TAG.remove(storage, asset_addr);
}