use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InitMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateTrigger {},
    DeleteTrigger {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
