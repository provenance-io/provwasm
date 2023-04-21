use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
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
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    // Create and save contract config state.
    config(deps.storage).save(&State {
        contract_name: msg.name.clone(),
    })?;

    // Create a message that will give the contract a name.
    let bind_name_msg = MsgBindNameRequest {
        parent: None,
        record: Some(NameRecord {
            name: msg.name.clone(),
            address: env.contract.address.into_string(),
            restricted: true,
        }),
    };

    // Dispatch message to handler and emit events
    let res = Response::new()
        .add_message(bind_name_msg)
        .add_attribute("integration_test", "v2")
        .add_attribute("action", "provwasm.contracts.scope.init")
        .add_attribute("name", msg.name);
    Ok(res)
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
    }
}

// Use a ProvenanceQuerier to get a scope by ID.
fn try_get_scope(deps: Deps, id: String) -> Result<QueryResponse, StdError> {
    let querier = MetadataQuerier::new(&deps.querier);
    let scope_response = querier.scope(id, "".to_string(), "".to_string(), false, false)?;
    to_binary(&scope_response)
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
    to_binary(&sessions_response)
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
    to_binary(&records_response)
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
    to_binary(&records_response)
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::from_binary;
//     use cosmwasm_std::testing::{mock_env, mock_info};
//
//     #[test]
//     fn valid_init() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//
//         // Init contract state
//         let res = instantiate(
//             deps.as_mut(),
//             mock_env(),
//             mock_info("sender", &[]),
//             InitMsg {
//                 name: "contract.pb".into(),
//             },
//         )
//         .unwrap(); // Panics on error
//
//         // Just check that one message was created (bind name to contract address).
//         assert_eq!(1, res.messages.len());
//     }
//
//     #[test]
//     fn query_scope() {
//         // Read a scope from file
//         let bin = must_read_binary_file("testdata/scope.json");
//         let expected: Scope = from_binary(&bin).unwrap();
//
//         // Create custom deps with the scope.
//         let mut deps = mock_dependencies(&[]);
//         deps.querier.with_scope(expected.clone());
//
//         // Call the contract query function.
//         let bin = query(
//             deps.as_ref(),
//             mock_env(),
//             QueryMsg::GetScope {
//                 id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
//             },
//         )
//         .unwrap();
//
//         // Ensure we got the expected scope.
//         let scope: Scope = from_binary(&bin).unwrap();
//         assert_eq!(scope, expected)
//     }
//
//     #[test]
//     fn query_sessions() {
//         // Read test metadata JSON files
//         let bin = must_read_binary_file("testdata/scope.json");
//         let scope: Scope = from_binary(&bin).unwrap();
//         let bin = must_read_binary_file("testdata/sessions.json");
//         let expected: Sessions = from_binary(&bin).unwrap();
//
//         // Create custom deps with metadata.
//         let mut deps = mock_dependencies(&[]);
//         deps.querier.with_sessions(scope, expected.clone());
//
//         // Call the contract query function.
//         let bin = query(
//             deps.as_ref(),
//             mock_env(),
//             QueryMsg::GetSessions {
//                 scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
//             },
//         )
//         .unwrap();
//
//         // Ensure we got the expected sessions.
//         let sessions: Sessions = from_binary(&bin).unwrap();
//         assert_eq!(sessions, expected)
//     }
//
//     #[test]
//     fn query_records() {
//         // Read test metadata JSON files
//         let bin = must_read_binary_file("testdata/scope.json");
//         let scope: Scope = from_binary(&bin).unwrap();
//         let bin = must_read_binary_file("testdata/records.json");
//         let expected: Records = from_binary(&bin).unwrap();
//
//         // Create custom deps with metadata.
//         let mut deps = mock_dependencies(&[]);
//         deps.querier.with_records(scope, expected.clone());
//
//         // Call the contract query function.
//         let bin = query(
//             deps.as_ref(),
//             mock_env(),
//             QueryMsg::GetRecords {
//                 scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
//             },
//         )
//         .unwrap();
//
//         // Ensure we got the expected records
//         let records: Records = from_binary(&bin).unwrap();
//         assert_eq!(records, expected)
//     }
//
//     #[test]
//     fn query_record_by_name() {
//         // Read test metadata JSON files
//         let bin = must_read_binary_file("testdata/scope.json");
//         let scope: Scope = from_binary(&bin).unwrap();
//         let bin = must_read_binary_file("testdata/records.json");
//         let records: Records = from_binary(&bin).unwrap();
//         let bin = must_read_binary_file("testdata/loan_record.json");
//         let expected: Record = from_binary(&bin).unwrap();
//
//         // Create custom deps with metadata.
//         let mut deps = mock_dependencies(&[]);
//         deps.querier.with_records(scope, records);
//
//         // Call the contract query function.
//         let bin = query(
//             deps.as_ref(),
//             mock_env(),
//             QueryMsg::GetRecordByName {
//                 scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
//                 name: "loan".into(),
//             },
//         )
//         .unwrap();
//
//         // Ensure we got the expected record.
//         let record: Record = from_binary(&bin).unwrap();
//         assert_eq!(record, expected);
//     }
// }
