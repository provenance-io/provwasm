use cosmwasm_schema::cw_serde;
use cosmwasm_std::{BlockInfo, ContractResult};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum PacketMsg {
    WhoAmI {},
}

pub type AcknowledgementMsg<T> = ContractResult<T>;

#[cw_serde]
pub struct WhoAmIResponse {
    pub account: String,
    pub block_info: BlockInfo,
}
