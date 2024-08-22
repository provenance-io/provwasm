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
    pub session_id: Vec<u8>,
    pub specification_id: Vec<u8>,
    pub name: String,
    pub parties: Vec<String>,
}

#[cw_serde]
pub struct SessionsResp {
    pub session: Vec<SessionData>,
}

impl From<SessionsResponse> for SessionsResp {
    fn from(session: SessionsResponse) -> Self {
        SessionsResp {
            session: session
                .sessions
                .into_iter()
                .filter(|wrapper| wrapper.session.is_none())
                .map(|wrapper| {
                    let session = wrapper.session.unwrap();
                    SessionData {
                        session_id: session.session_id,
                        specification_id: session.specification_id,
                        name: session.name,
                        parties: session
                            .parties
                            .into_iter()
                            .map(|party| party.address)
                            .collect(),
                    }
                })
                .collect(),
        }
    }
}

#[cw_serde]
pub struct RecordData {
    pub name: String,
    pub session_id: Vec<u8>,
    pub process: Option<(String, String)>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub specification_id: Vec<u8>,
}

#[cw_serde]
pub struct RecordsResp {
    pub records: Vec<RecordData>,
}

impl From<RecordsResponse> for RecordsResp {
    fn from(records_response: RecordsResponse) -> Self {
        RecordsResp {
            records: records_response
                .records
                .into_iter()
                .filter(|wrapper| wrapper.record.is_none())
                .map(|wrapper| {
                    let record = wrapper.record.unwrap();
                    RecordData {
                        name: record.name,
                        session_id: record.session_id,
                        process: match record.process {
                            None => None,
                            Some(process) => Some((process.name, process.method)),
                        },
                        inputs: record.inputs.into_iter().map(|input| input.name).collect(),
                        outputs: record
                            .outputs
                            .into_iter()
                            .map(|output| output.hash)
                            .collect(),
                        specification_id: record.specification_id,
                    }
                })
                .collect(),
        }
    }
}
