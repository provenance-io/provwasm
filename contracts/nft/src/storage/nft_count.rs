use cosmwasm_std::Storage;
use cw_storage_plus::Item;

use crate::core::constants::NFT_COUNT_KEY;
use crate::core::error::ContractError;

pub const NFT_COUNT: Item<u64> = Item::new(NFT_COUNT_KEY);

pub fn nft_count(storage: &dyn Storage) -> Result<u64, ContractError> {
    Ok(NFT_COUNT.may_load(storage)?.unwrap_or_default())
}

pub fn increment_nft_count(storage: &mut dyn Storage) -> Result<u64, ContractError> {
    let val = nft_count(storage)? + 1;
    NFT_COUNT.save(storage, &val)?;
    Ok(val)
}

pub fn decrement_nft_count(storage: &mut dyn Storage) -> Result<u64, ContractError> {
    let val = nft_count(storage)? - 1;
    NFT_COUNT.save(storage, &val)?;
    Ok(val)
}
