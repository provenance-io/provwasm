use cosmwasm_std::{Addr, BlockInfo};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use cw_utils::Expiration;
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
