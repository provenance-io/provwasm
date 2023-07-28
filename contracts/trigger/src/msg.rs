use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

#[cw_serde]
pub enum Event {
    BlockHeightEvent { block_height: Uint64 },
    BlockTimeEvent { timestamp: Uint64 },
}

#[cw_serde]
pub struct InitMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateTrigger { event: Event, to_address: String },
    DeleteTrigger { id: Uint64 },
}
