use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;

/// A message sent to initialize the contract state.
#[cw_serde]
pub struct InitMsg {
    pub contract_name: String,
    pub purchase_denom: String,
    pub merchant_address: String,
    pub fee_percent: Decimal,
}

/// A message sent to transfer funds and collect fees for a purchase.
#[cw_serde]
pub enum ExecuteMsg {
    Purchase { id: String },
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}

/// A message sent to query contract config state.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::State)]
    QueryRequest {},
}
