use crate::core::constants::OPERATORS_KEY;
use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use cw_utils::Expiration;

pub const OPERATORS: Map<'static, (&'static Addr, &'static Addr), Expiration> =
    Map::new(OPERATORS_KEY);
