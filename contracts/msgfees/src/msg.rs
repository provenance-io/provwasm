use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InitMsg {
    pub fee_amount: Option<Coin>,
    pub fee_recipient: Option<Addr>,
}

#[cw_serde]
pub enum ExecuteMsg {
    SendFunds { funds: Coin, to_address: Addr },
}
