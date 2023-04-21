use cosmwasm_schema::{cw_serde, QueryResponses};
use provwasm_std::types::provenance::metadata::v1::Scope;

#[cw_serde]
pub struct InitMsg {
    pub name: String, // Bind this name to the contract address (eg contract.pb).
}

#[cw_serde]
pub enum ExecuteMsg {
    WriteScope { scope: Scope, signers: Vec<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(provwasm_std::types::provenance::metadata::v1::Scope)]
    GetScope { id: String },
    #[returns(provwasm_std::types::provenance::metadata::v1::Session)]
    GetSessions { scope_id: String },
    #[returns(Vec<provwasm_std::types::provenance::metadata::v1::Record>)]
    GetRecords { scope_id: String },
    #[returns(provwasm_std::types::provenance::metadata::v1::Record)]
    GetRecordByName { scope_id: String, name: String },
}
