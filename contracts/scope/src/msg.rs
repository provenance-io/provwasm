use cosmwasm_schema::{cw_serde, QueryResponses};
use provwasm_std::types::provenance::metadata::v1::{
    ContractSpecificationResponse, PartyType, RecordsResponse, Scope, ScopeResponse,
    SessionsResponse,
};

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
    pub owner_addresses: Vec<String>,
    pub parties_involved: Vec<String>,
}

#[cw_serde]
pub struct ContractSpecResp {
    pub contract_spec: Option<ContractSpecData>,
}

impl From<ContractSpecificationResponse> for ContractSpecResp {
    fn from(contract: ContractSpecificationResponse) -> Self {
        ContractSpecResp {
            contract_spec: match contract.contract_specification {
                None => None,
                Some(spec) => match spec.specification {
                    None => None,
                    Some(spec) => Some(ContractSpecData {
                        id: spec.specification_id,
                        class_name: spec.class_name,
                        owner_addresses: spec.owner_addresses,
                        parties_involved: spec
                            .parties_involved
                            .into_iter()
                            .map(|party| {
                                PartyType::try_from(party)
                                    .unwrap()
                                    .as_str_name()
                                    .to_string()
                            })
                            .collect(),
                    }),
                },
            },
        }
    }
}

#[cw_serde]
pub struct ScopeData {
    pub scope_id: Vec<u8>,
    pub specification_id: Vec<u8>,
    pub owners: Vec<String>,
    pub value_owner_address: String,
}

#[cw_serde]
pub struct ScopeResp {
    pub scope: Option<ScopeData>,
}

impl From<ScopeResponse> for ScopeResp {
    fn from(scope_response: ScopeResponse) -> Self {
        ScopeResp {
            scope: match scope_response.scope {
                None => None,
                Some(wrapper) => match wrapper.scope {
                    None => None,
                    Some(scope) => Some(ScopeData {
                        scope_id: scope.scope_id,
                        specification_id: scope.specification_id,
                        owners: scope
                            .owners
                            .into_iter()
                            .map(|owner| owner.address)
                            .collect(),
                        value_owner_address: scope.value_owner_address,
                    }),
                },
            },
        }
    }
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
