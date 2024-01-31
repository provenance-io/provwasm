use cosmwasm_std::{
    entry_point, to_json_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
};
use provwasm_std::types::provenance::metadata::v1::{MetadataQuerier, MsgWriteScopeRequest, Scope};
use provwasm_std::types::provenance::name::v1::{MsgBindNameRequest, NameRecord};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InitMsg, QueryMsg};
use crate::state::{config, State};

/// Initialize config state and bind a name to the contract address.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    // Create and save contract config state.
    config(deps.storage).save(&State {
        contract_name: msg.name.clone(),
    })?;

    let split: Vec<&str> = msg.name.splitn(2, '.').collect();
    let record = split.first();
    let parent = split.last();

    match (parent, record) {
        (Some(parent), Some(record)) => {
            // Create a bind name message
            let bind_name_msg = MsgBindNameRequest {
                parent: Some(NameRecord {
                    name: parent.to_string(),
                    address: env.contract.address.to_string(),
                    restricted: true,
                }),
                record: Some(NameRecord {
                    name: record.to_string(),
                    address: env.contract.address.to_string(),
                    restricted: true,
                }),
            };

            // Dispatch bind name message and add event attributes.
            let res = Response::new()
                .add_message(bind_name_msg)
                .add_attribute("integration_test", "v2")
                .add_attribute("action", "provwasm.contracts.scope.init")
                .add_attribute("contract_name", msg.name)
                .add_attribute("contract_owner", info.sender);
            Ok(res)
        }
        (_, _) => Err(ContractError::Std(StdError::GenericErr {
            msg: "Invalid contract name".to_string(),
        })),
    }
}

/// Handle scope execute requests for the provenance metadata module.
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::WriteScope { scope, signers } => try_write_scope(deps, scope, signers),
    }
}

/// Create and dispatch a provenance message that will write scope data
pub fn try_write_scope(
    _deps: DepsMut,
    scope: Scope,
    signers: Vec<String>,
) -> Result<Response, StdError> {
    Ok(Response::new().add_message(MsgWriteScopeRequest {
        scope: Some(scope),
        signers,
        scope_uuid: "".to_string(),
        spec_uuid: "".to_string(),
    }))
}

/// Handle scope query requests for the provenance metadata module.
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetScope { id } => try_get_scope(deps, id),
        QueryMsg::GetSessions { scope_id } => try_get_sessions(deps, scope_id),
        QueryMsg::GetRecords { scope_id } => try_get_records(deps, scope_id),
        QueryMsg::GetRecordByName { scope_id, name } => {
            try_get_records_by_name(deps, scope_id, name)
        }
        QueryMsg::GetContractSpec { id } => try_get_contract_spec(deps, id),
    }
}

// Use a ProvenanceQuerier to get a scope by ID.
fn try_get_contract_spec(deps: Deps, id: String) -> Result<QueryResponse, StdError> {
    let querier = MetadataQuerier::new(&deps.querier);
    let contract_spec_response = querier.contract_specification(id, true)?;
    to_json_binary(&contract_spec_response)
}

// Use a ProvenanceQuerier to get a scope by ID.
fn try_get_scope(deps: Deps, id: String) -> Result<QueryResponse, StdError> {
    let querier = MetadataQuerier::new(&deps.querier);
    let scope_response = querier.scope(id, "".to_string(), "".to_string(), false, false)?;
    to_json_binary(&scope_response)
}

// Use a ProvenanceQuerier to get sessions for a scope.
fn try_get_sessions(deps: Deps, scope_id: String) -> Result<QueryResponse, StdError> {
    let querier = MetadataQuerier::new(&deps.querier);
    let sessions_response = querier.sessions(
        scope_id,
        "".to_string(),
        "".to_string(),
        "".to_string(),
        false,
        false,
    )?;
    to_json_binary(&sessions_response)
}

// Use a ProvenanceQuerier to get records for a scope.
fn try_get_records(deps: Deps, scope_id: String) -> Result<QueryResponse, StdError> {
    let querier = MetadataQuerier::new(&deps.querier);
    let records_response = querier.records(
        "".to_string(),
        scope_id,
        "".to_string(),
        "".to_string(),
        false,
        false,
    )?;
    to_json_binary(&records_response)
}

// Use a ProvenanceQuerier to get records for a scope by name
fn try_get_records_by_name(
    deps: Deps,
    scope_id: String,
    name: String,
) -> Result<QueryResponse, StdError> {
    let querier = MetadataQuerier::new(&deps.querier);
    let records_response =
        querier.records("".to_string(), scope_id, "".to_string(), name, false, false)?;
    to_json_binary(&records_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::from_json;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use provwasm_mocks::mock_provenance_dependencies;
    use provwasm_std::types::provenance::metadata::v1::process::ProcessId;
    use provwasm_std::types::provenance::metadata::v1::record_input::Source;
    use provwasm_std::types::provenance::metadata::v1::{
        Party, PartyType, Process, Record, RecordInput, RecordInputStatus, RecordOutput,
        RecordWrapper, RecordsRequest, RecordsResponse, ResultStatus, ScopeRequest, ScopeResponse,
        ScopeWrapper, Session, SessionWrapper, SessionsRequest, SessionsResponse,
    };

    #[test]
    fn valid_init() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();

        // Init contract state
        let res = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Just check that one message was created (bind name to contract address).
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn query_scope() {
        let expected = ScopeResponse {
            scope: Some(ScopeWrapper {
                scope: Some(Scope {
                    scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d"
                        .as_bytes()
                        .to_vec(),
                    specification_id: "scopespec1qj5hx4l3vgryhp5g3ks68wh53jkq3net7n"
                        .as_bytes()
                        .to_vec(),
                    owners: vec![Party {
                        address: "tp1rr4d0eu62pgt4edw38d2ev27798pfhdhp5ttha".to_string(),
                        role: PartyType::Originator.into(),
                        optional: false,
                    }],
                    data_access: vec!["tp1rr4d0eu62pgt4edw38d2ev27798pfhdhp5ttha".to_string()],
                    value_owner_address: "tp1r7dd2w80x939u5f3l5y04rruusyr5dxx7wuvdm".to_string(),
                    require_party_rollup: false,
                }),
                scope_id_info: None,
                scope_spec_id_info: None,
            }),
            sessions: vec![],
            records: vec![],
            request: None,
        };

        // Create custom deps with the scope.
        let mut deps = mock_provenance_dependencies();

        ScopeRequest::mock_response(&mut deps.querier, expected.clone());

        // Call the contract query function.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetScope {
                id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
            },
        )
        .unwrap();

        // Ensure we got the expected scope.
        let scope: ScopeResponse = from_json(&bin).unwrap();
        assert_eq!(scope, expected)
    }

    #[test]
    fn query_sessions() {
        let expected = SessionsResponse {
            scope: None,
            sessions: vec![SessionWrapper {
                session: Some(Session {
                    session_id:
                        "session1qyqqq2wf3c4yt4u447m8pw65qcdxx9k7nswc7sec43qcmpp9267xw6r8xv4"
                            .as_bytes()
                            .to_vec(),
                    specification_id: "contractspec1qd5xsyufyuyh7a5ejg7n2d336x2ql87dl2"
                        .as_bytes()
                        .to_vec(),
                    parties: vec![Party {
                        address: "tp1rr4d0eu62pgt4edw38d2ev27798pfhdhp5ttha".to_string(),
                        role: PartyType::Originator.into(),
                        optional: false,
                    }],
                    name: "io.p8e.contracts.origination.RecordPropertyLoanETLContract".to_string(),
                    context: "CiYKJGYwYTJmMmJhLTBiNWYtNDY0OC05NGM1LTgwZjlhN2U4YTUxNg=="
                        .as_bytes()
                        .to_vec(),
                    audit: None,
                }),
                session_id_info: None,
                contract_spec_id_info: None,
            }],
            records: vec![],
            request: None,
        };

        // Create custom deps with metadata.
        let mut deps = mock_provenance_dependencies();

        SessionsRequest::mock_response(&mut deps.querier, expected.clone());

        // Call the contract query function.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetSessions {
                scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
            },
        )
        .unwrap();

        // Ensure we got the expected sessions.
        let sessions: SessionsResponse = from_json(&bin).unwrap();
        assert_eq!(sessions, expected)
    }

    #[test]
    fn query_records() {
        let expected = RecordsResponse{
            scope: None,
            sessions: vec![],
            records: vec![
                RecordWrapper{
                    record: Some(Record{
                        name: "tri_merge_reports".to_string(),
                        session_id: "session1qyqqq2wf3c4yt4u447m8pw65qcdxx9k7nswc7sec43qcmpp9267xw6r8xv4".as_bytes().to_vec(),
                        process: Some(
                            Process{
                                name: "io.provenance.proto.loan.LoanProtos$TriMergeReportsList".to_string(),
                                method: "tri_merge_reports".to_string(),
                                process_id: Some(ProcessId::Hash("mCVay1v4K017VMs2EIbLjuLS2OcOVTAn7LjrKvqy5UZoBXA+dDdyeElWPXKdiw8OOzx5UjOfa/mbla1/PlzZyw==".to_string())),
                            }
                        ),
                        inputs: vec![
                            RecordInput{
                                name: "perform_input_checks".to_string(),
                                type_name: "io.p8e.proto.Common$BooleanResult".to_string(),
                                status: RecordInputStatus::Proposed.into(),
                                source: Some(Source::Hash("z4PhNX7vuL3xVChQ1m2AB9Yg5AULVxXcg/SpIdNs6c5H0NE8XYXysP+DGNKHfuwvY7kxvUdBeoGlODJ6+SfaPg==".to_string())),
                            },
                             RecordInput {
                                 name: "tri_merge_reports".to_string(),
                                 type_name: "io.provenance.proto.loan.LoanProtos$TriMergeReportsList".to_string(),
                                 status: RecordInputStatus::Proposed.into(),
                                 source: Some(Source::Hash("T/SPBHyZ7fZ3YhPra7e/ObJdHG/QC3xUSjbxl261b9eAGTt6ETrtSgJRQmzJbK4N57iQKpsf3PtDV2jo8rWBKw==".to_string())),
                             }
                        ],
                        outputs: vec![RecordOutput{
                            hash: "T/SPBHyZ7fZ3YhPra7e/ObJdHG/QC3xUSjbxl261b9eAGTt6ETrtSgJRQmzJbK4N57iQKpsf3PtDV2jo8rWBKw==".to_string(),
                            status: ResultStatus::Pass.into(),
                        }],
                        specification_id: "recspec1q45xsyufyuyh7a5ejg7n2d336x2qd6lrzs702zd4yh02s5p4tcwmgwxxy3q".as_bytes().to_vec(),
                    }),
                    record_id_info: None,
                    record_spec_id_info: None,
                },
                RecordWrapper{
                    record: Some(Record{
                        name: "blockchain_custody".to_string(),
                        session_id: "session1qyqqq2wf3c4yt4u447m8pw65qcdxx9k7nswc7sec43qcmpp9267xw6r8xv4".as_bytes().to_vec(),
                        process: Some(
                            Process{
                                name: "io.provenance.proto.loan.LoanProtos$BlockchainCustody".to_string(),
                                method: "blockchain_custody".to_string(),
                                process_id: Some(ProcessId::Hash("mCVay1v4K017VMs2EIbLjuLS2OcOVTAn7LjrKvqy5UZoBXA+dDdyeElWPXKdiw8OOzx5UjOfa/mbla1/PlzZyw==".to_string())),
                            }
                        ),
                        inputs: vec![
                            RecordInput{
                                name: "perform_input_checks".to_string(),
                                type_name: "io.p8e.proto.Common$BooleanResult".to_string(),
                                status: RecordInputStatus::Proposed.into(),
                                source: Some(Source::Hash("z4PhNX7vuL3xVChQ1m2AB9Yg5AULVxXcg/SpIdNs6c5H0NE8XYXysP+DGNKHfuwvY7kxvUdBeoGlODJ6+SfaPg==".to_string())),
                            },
                             RecordInput {
                                 name: "blockchain_custody".to_string(),
                                 type_name: "io.provenance.proto.loan.LoanProtos$BlockchainCustody".to_string(),
                                 status: RecordInputStatus::Proposed.into(),
                                 source: Some(Source::Hash("y5X8TGgn+Vg7I5fUL63OQVRjMSZPI+b59KlbsftAUQ8a01a3QjfiZ66+i346HsxhyrhvH61Ro1Gra+0f1CiNVg==".to_string())),
                             }
                        ],
                        outputs: vec![RecordOutput{
                            hash: "y5X8TGgn+Vg7I5fUL63OQVRjMSZPI+b59KlbsftAUQ8a01a3QjfiZ66+i346HsxhyrhvH61Ro1Gra+0f1CiNVg==".to_string(),
                            status: ResultStatus::Pass.into(),
                        }],
                        specification_id: "recspec1q45xsyufyuyh7a5ejg7n2d336x2qd6lrzs702zd4yh02s5p4tcwmgwxxy3q".as_bytes().to_vec(),
                    }),
                    record_id_info: None,
                    record_spec_id_info: None,
                }],
            request: None,
        };

        // Create custom deps with metadata.
        let mut deps = mock_provenance_dependencies();

        RecordsRequest::mock_response(&mut deps.querier, expected.clone());

        // Call the contract query function.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetRecords {
                scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
            },
        )
        .unwrap();

        // Ensure we got the expected records
        let records: RecordsResponse = from_json(&bin).unwrap();
        assert_eq!(records, expected)
    }

    #[test]
    fn query_record_by_name() {
        let expected = RecordsResponse{
            scope: None,
            sessions: vec![],
            records: vec![
                RecordWrapper{
                    record: Some(Record{
                        name: "loan".to_string(),
                        session_id: "session1qyqqq2wf3c4yt4u447m8pw65qcdxx9k7nswc7sec43qcmpp9267xw6r8xv4".as_bytes().to_vec(),
                        process: Some(
                            Process{
                                name: "io.provenance.proto.loan.LoanProtos$Loan".to_string(),
                                method: "loan".to_string(),
                                process_id: Some(ProcessId::Hash("mCVay1v4K017VMs2EIbLjuLS2OcOVTAn7LjrKvqy5UZoBXA+dDdyeElWPXKdiw8OOzx5UjOfa/mbla1/PlzZyw==".to_string())),
                            }
                        ),
                        inputs: vec![
                            RecordInput{
                                name: "perform_input_checks".to_string(),
                                type_name: "io.p8e.proto.Common$BooleanResult".to_string(),
                                status: RecordInputStatus::Proposed.into(),
                                source: Some(Source::Hash("z4PhNX7vuL3xVChQ1m2AB9Yg5AULVxXcg/SpIdNs6c5H0NE8XYXysP+DGNKHfuwvY7kxvUdBeoGlODJ6+SfaPg==".to_string())),
                            },
                            RecordInput {
                                name: "loan".to_string(),
                                type_name: "io.provenance.proto.loan.LoanProtos$Loan".to_string(),
                                status: RecordInputStatus::Proposed.into(),
                                source: Some(Source::Hash("poYoiYr8gi22vyBlo09YkSSnGHRY0jQW9DZAvaTPT5slTbt2SV8KgGSKoK72PYVL/yLrCgnrEDaRn08byB/JHQ==".to_string())),
                            }
                        ],
                        outputs: vec![RecordOutput{
                            hash: "poYoiYr8gi22vyBlo09YkSSnGHRY0jQW9DZAvaTPT5slTbt2SV8KgGSKoK72PYVL/yLrCgnrEDaRn08byB/JHQ==".to_string(),
                            status: ResultStatus::Pass.into(),
                        }],
                        specification_id: "recspec1q45xsyufyuyh7a5ejg7n2d336x2yw2alzjfrutnualv99xp9csq7sdqwhln".as_bytes().to_vec(),
                    }),
                    record_id_info: None,
                    record_spec_id_info: None,
                }],
            request: None,
        };

        // Create custom deps with metadata.
        let mut deps = mock_provenance_dependencies();

        RecordsRequest::mock_response(&mut deps.querier, expected.clone());

        // Call the contract query function.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetRecordByName {
                scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
                name: "loan".into(),
            },
        )
        .unwrap();

        // Ensure we got the expected record.
        let record: RecordsResponse = from_json(&bin).unwrap();
        assert_eq!(record, expected);
    }
}
