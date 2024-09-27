use cosmwasm_schema::{cw_serde, QueryResponses};
use provwasm_std::types::provenance::name::v1::QueryReverseLookupResponse;

#[cw_serde]
pub struct InitMsg {
    pub name: String, // Bind this name to the contract address (eg contract.pb).
}

#[cw_serde]
pub enum ExecuteMsg {
    BindPrefix { prefix: String },
    UnbindPrefix { prefix: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    Resolve { name: String },
    #[returns(LookupResponse)]
    Lookup { address: String },
    #[returns(String)]
    Params {},
}

#[cw_serde]
pub struct LookupResponse {
    pub name: Vec<String>,
}

impl From<QueryReverseLookupResponse> for LookupResponse {
    fn from(value: QueryReverseLookupResponse) -> Self {
        LookupResponse { name: value.name }
    }
}
