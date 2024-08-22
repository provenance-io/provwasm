use cosmwasm_schema::{cw_serde, QueryResponses};
use provwasm_std::types::provenance::metadata::v1::Scope;

#[cw_serde]
pub struct InitMsg {
    pub name: String, // Bind this name to the contract address (eg contract.pb).
}

#[cw_serde]
pub enum ExecuteMsg {
    WriteScope {
        scope: ScopeData,
        signers: Vec<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ContractSpecResp)]
    GetContractSpec { id: String },
    #[returns(ScopeResp)]
    GetScope { id: String },
    #[returns(SessionsResp)]
    GetSessions { scope_id: String },
    #[returns(Vec<RecordsResp>)]
    GetRecords { scope_id: String },
    #[returns(RecordsResp)]
    GetRecordByName { scope_id: String, name: String },
}

#[cw_serde]
pub struct ContractSpecData {
    pub id: Vec<u8>,
    pub class_name: String,
    pub description: String,
    pub owner_addresses: Vec<String>,
    pub party_type: String,
}

#[cw_serde]
pub struct ContractSpecResp {
    pub contract_spec: Option<ContractSpecData>,
}

#[cw_serde]
pub struct ScopeData {
    pub scope_id: String,
    pub specification_id: String,
    pub owners: Vec<String>,
    pub value_owner_address: String,
}

#[cw_serde]
pub struct ScopeResp {
    pub scope: Option<ScopeData>,
}

#[cw_serde]
pub struct SessionData {
    pub session_id: String,
    pub specification_id: String,
    pub name: String,
    pub data_access: Vec<String>,
}

#[cw_serde]
pub struct SessionsResp {
    pub session: Vec<SessionData>,
}

#[cw_serde]
pub struct RecordsData {
    pub name: String,
    pub session_id: String,
    pub process_name: Option<String>,
    pub process_method: Option<String>,
}

#[cw_serde]
pub struct RecordsResp {
    pub records: Vec<RecordsData>,
    pub inputs: Vec<String>,
    pub output: String,
    pub specification_id: String,
}
