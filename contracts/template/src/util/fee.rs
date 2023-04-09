use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct Fee {
    pub recipient: Option<Addr>,
    pub amount: Coin,
}
