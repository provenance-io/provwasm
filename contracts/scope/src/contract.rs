use cosmwasm_std::{
    to_binary, Addr, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
};

use provwasm_std::{
    bind_name, write_scope, NameBinding, ProvenanceMsg, ProvenanceQuerier, ProvenanceQuery, Record,
    Records, Scope, Sessions,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InitMsg, QueryMsg};
use crate::state::{config, State};

/// Initialize config state and bind a name to the contract address.
pub fn instantiate(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Create and save contract config state.
    config(deps.storage).save(&State {
        contract_name: msg.name.clone(),
    })?;

    // Create a message that will give the contract a name.
    let bind_name_msg = bind_name(
        msg.name.clone(),
        env.contract.address,
        NameBinding::Restricted,
    )?;

    // Dispatch message to handler and emit events
    let res: Response<ProvenanceMsg> = Response::new()
        .add_message(bind_name_msg)
        .add_attribute("integration_test", "v2")
        .add_attribute("action", "provwasm.contracts.scope.init")
        .add_attribute("name", msg.name);
    Ok(res)
}

/// Handle scope execute requests for the provenance metadata module.
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ProvenanceMsg>, StdError> {
    match msg {
        ExecuteMsg::WriteScope { scope, signers } => try_write_scope(deps, scope, signers),
    }
}

/// Create and dispatch a provenance message that will write scope data
pub fn try_write_scope(
    _deps: DepsMut,
    scope: Scope,
    signers: Vec<Addr>,
) -> Result<Response<ProvenanceMsg>, StdError> {
    Ok(Response::new().add_message(write_scope(scope, signers)?))
}

/// Handle scope query requests for the provenance metadata module.
pub fn query(
    deps: Deps<ProvenanceQuery>,
    _env: Env,
    msg: QueryMsg,
) -> Result<QueryResponse, StdError> {
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
fn try_get_scope(deps: Deps<ProvenanceQuery>, id: String) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let scope: Scope = querier.get_scope(id)?;
    to_binary(&scope)
}

// Use a ProvenanceQuerier to get sessions for a scope.
fn try_get_sessions(
    deps: Deps<ProvenanceQuery>,
    scope_id: String,
) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let sessions: Sessions = querier.get_sessions(scope_id)?;
    to_binary(&sessions)
}

// Use a ProvenanceQuerier to get records for a scope.
fn try_get_records(
    deps: Deps<ProvenanceQuery>,
    scope_id: String,
) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let records: Records = querier.get_records(scope_id)?;
    to_binary(&records)
}

// Use a ProvenanceQuerier to get records for a scope by name
fn try_get_records_by_name(
    deps: Deps<ProvenanceQuery>,
    scope_id: String,
    name: String,
) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let record: Record = querier.get_record_by_name(scope_id, name)?;
    to_binary(&record)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use provwasm_mocks::{mock_dependencies, must_read_binary_file};

    #[test]
    fn valid_init() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);

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
        // Read a scope from file
        let bin = must_read_binary_file("testdata/scope.json");
        let expected: Scope = from_binary(&bin).unwrap();

        // Create custom deps with the scope.
        let mut deps = mock_dependencies(&[]);
        deps.querier.with_scope(expected.clone());

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
        let scope: Scope = from_binary(&bin).unwrap();
        assert_eq!(scope, expected)
    }

    #[test]
    fn query_sessions() {
        // Read test metadata JSON files
        let bin = must_read_binary_file("testdata/scope.json");
        let scope: Scope = from_binary(&bin).unwrap();
        let bin = must_read_binary_file("testdata/sessions.json");
        let expected: Sessions = from_binary(&bin).unwrap();

        // Create custom deps with metadata.
        let mut deps = mock_dependencies(&[]);
        deps.querier.with_sessions(scope, expected.clone());

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
        let sessions: Sessions = from_binary(&bin).unwrap();
        assert_eq!(sessions, expected)
    }

    #[test]
    fn query_records() {
        // Read test metadata JSON files
        let bin = must_read_binary_file("testdata/scope.json");
        let scope: Scope = from_binary(&bin).unwrap();
        let bin = must_read_binary_file("testdata/records.json");
        let expected: Records = from_binary(&bin).unwrap();

        // Create custom deps with metadata.
        let mut deps = mock_dependencies(&[]);
        deps.querier.with_records(scope, expected.clone());

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
        let records: Records = from_binary(&bin).unwrap();
        assert_eq!(records, expected)
    }

    #[test]
    fn query_record_by_name() {
        // Read test metadata JSON files
        let bin = must_read_binary_file("testdata/scope.json");
        let scope: Scope = from_binary(&bin).unwrap();
        let bin = must_read_binary_file("testdata/records.json");
        let records: Records = from_binary(&bin).unwrap();
        let bin = must_read_binary_file("testdata/loan_record.json");
        let expected: Record = from_binary(&bin).unwrap();

        // Create custom deps with metadata.
        let mut deps = mock_dependencies(&[]);
        deps.querier.with_records(scope, records);

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
        let record: Record = from_binary(&bin).unwrap();
        assert_eq!(record, expected);
    }
}
