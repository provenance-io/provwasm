use cosmwasm_std::Addr;
use provwasm_std::Scope;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {
    pub name: String, // Bind this name to the contract address (eg contract.pb).
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    WriteScope { scope: Scope, signers: Vec<Addr> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetScope { id: String },
    GetSessions { scope_id: String },
    GetRecords { scope_id: String },
    GetRecordByName { scope_id: String, name: String },
}
