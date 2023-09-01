use cosmwasm_std::{Addr, BlockInfo};
use cw721::Expiration;
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::constants::NFT_KEY;
use crate::core::constants::NFT_OWNER_KEY;

// pub const NFT: Item<Nft> = Item::new(NFT_KEY);
pub const INDEXES: TokenIndexes = TokenIndexes {
    owner: MultiIndex::new(token_owner_idx, NFT_KEY, NFT_OWNER_KEY),
};
pub const TOKENS: IndexedMap<'static, &'static str, Nft, TokenIndexes<'static>> =
    IndexedMap::new(NFT_KEY, INDEXES);

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: Addr,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

impl Approval {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        self.expires.is_expired(block)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Nft {
    // using scope id here
    pub id: String,
    pub owner: Addr,
    pub approvals: Vec<Approval>,
}

impl Nft {
    pub fn new(scope_uuid: String, owner: Addr) -> Self {
        Nft {
            id: scope_uuid,
            owner,
            approvals: vec![],
        }
    }
}

// pub fn get(storage: &dyn Storage) -> Result<Nft, ContractError> {
//     Ok(NFT.load(storage)?)
// }
//
// pub fn set(storage: &mut dyn Storage, state: &Nft) -> Result<(), ContractError> {
//     Ok(NFT.save(storage, state)?)
// }

pub struct TokenIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, Nft, String>,
}

impl<'a> IndexList<Nft> for TokenIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Nft>> + '_> {
        let v: Vec<&dyn Index<Nft>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn token_owner_idx(_pk: &[u8], d: &Nft) -> Addr {
    d.owner.clone()
}
//
// #[cfg(test)]
// mod tests {
//     use cosmwasm_std::Addr;
//     use provwasm_mocks::mock_provenance_dependencies;
//
//     use crate::storage::nft::set;
//     use crate::storage::nft::Nft;
//
//     use super::get;
//
//     #[test]
//     fn test_invalid_get() {
//         let deps = mock_provenance_dependencies();
//         get(&deps.storage).unwrap_err();
//     }
//
//     #[test]
//     fn test_get_set() {
//         let mut deps = mock_provenance_dependencies();
//         let expected = Nft::new("scope_uuid".to_string(), Addr::unchecked("owner"));
//         set(deps.as_mut().storage, &expected).unwrap();
//         let nft = get(&deps.storage).unwrap();
//         assert_eq!(expected, nft);
//     }
// }
