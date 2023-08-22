use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

use crate::core::{constants::NFT_KEY, error::ContractError};

pub const NFT: Item<Nft> = Item::new(NFT_KEY);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Nft {
    // using scope id here
    pub id: String,
    pub owner: Addr,
}

impl Nft {
    pub fn new(scope_uuid: String, owner: Addr) -> Self {
        Nft {
            id: scope_uuid,
            owner,
        }
    }
}

pub fn get(storage: &dyn Storage) -> Result<Nft, ContractError> {
    Ok(NFT.load(storage)?)
}

pub fn set(storage: &mut dyn Storage, state: &Nft) -> Result<(), ContractError> {
    Ok(NFT.save(storage, state)?)
}

#[cfg(test)]
mod tests {
    use crate::storage::nft::Nft;
    use cosmwasm_std::Addr;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::storage::nft::set;

    use super::get;

    #[test]
    fn test_invalid_get() {
        let deps = mock_provenance_dependencies();
        get(&deps.storage).unwrap_err();
    }

    #[test]
    fn test_get_set() {
        let mut deps = mock_provenance_dependencies();
        let expected = Nft::new("scope_uuid".to_string(), Addr::unchecked("owner"));
        set(deps.as_mut().storage, &expected).unwrap();
        let nft = get(&deps.storage).unwrap();
        assert_eq!(expected, nft);
    }
}
