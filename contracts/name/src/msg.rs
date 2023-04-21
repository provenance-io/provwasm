use cosmwasm_schema::{cw_serde, QueryResponses};

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
    #[returns(provwasm_std::types::provenance::name::v1::QueryResolveResponse)]
    Resolve { name: String },
    #[returns(Vec<String>)]
    Lookup { address: String },
}
