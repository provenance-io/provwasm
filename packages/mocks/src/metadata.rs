use crate::common::{query_error, query_result};
use cosmwasm_std::{to_binary, QuerierResult};
use provwasm_std::{MetadataQueryParams, Records, Scope, Sessions};
use std::collections::HashMap;

/// A mock for testing provenance metadata module queries.
#[derive(Clone, Default)]
pub struct MetadataQuerier {
    scope_store: HashMap<String, Scope>,
    sessions_store: HashMap<String, Sessions>,
    records_store: HashMap<String, Records>,
}

impl MetadataQuerier {
    /// Create a new mock metdata querier with some scope metadata set. It is assumed that if
    /// sessions and records are passed, they are for the provided scope.
    pub fn new(scope: Scope, sessions: Option<Sessions>, records: Option<Records>) -> Self {
        // Store scope
        let mut scope_store = HashMap::new();
        scope_store.insert(scope.scope_id.to_string(), scope.clone());
        // Store sessions
        let mut sessions_store = HashMap::new();
        sessions.into_iter().for_each(|s| {
            sessions_store.insert(scope.scope_id.to_string(), s);
        });
        // Store records
        let mut records_store = HashMap::new();
        records.into_iter().for_each(|r| {
            records_store.insert(scope.scope_id.to_string(), r);
        });
        // Create and return the mock metadata querier
        MetadataQuerier {
            scope_store,
            sessions_store,
            records_store,
        }
    }

    fn get_scope(&self, scope_id: &str) -> Option<QuerierResult> {
        self.scope_store
            .get(&scope_id.to_string())
            .map(|scope| query_result(to_binary(scope)))
    }

    fn get_sessions(&self, scope_id: &str) -> Option<QuerierResult> {
        self.sessions_store
            .get(&scope_id.to_string())
            .map(|sessions| query_result(to_binary(sessions)))
    }

    fn get_records(&self, scope_id: &str, name: Option<String>) -> Option<QuerierResult> {
        self.records_store
            .get(&scope_id.to_string())
            .map(|r| Records {
                records: r
                    .records
                    .iter()
                    .filter(|rec| name.is_none() || rec.name == name.clone().unwrap())
                    .cloned()
                    .collect(),
            })
            .map(|records| query_result(to_binary(&records)))
    }

    pub fn query(&self, params: &MetadataQueryParams) -> QuerierResult {
        let maybe_result = match params {
            MetadataQueryParams::GetScope { scope_id } => self.get_scope(scope_id),
            MetadataQueryParams::GetSessions { scope_id } => self.get_sessions(scope_id),
            MetadataQueryParams::GetRecords { scope_id, name } => {
                self.get_records(scope_id, name.clone())
            }
        };
        match maybe_result {
            Some(r) => r,
            None => query_error("metadata not found", to_binary(params)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::must_read_binary_file;
    use cosmwasm_std::from_binary;

    #[test]
    fn query_scope() {
        let bin = must_read_binary_file("testdata/scope.json");
        let expected: Scope = from_binary(&bin).unwrap();
        let querier = MetadataQuerier::new(expected.clone(), None, None);

        let params = MetadataQueryParams::GetScope {
            scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let scope: Scope = from_binary(&bin).unwrap();

        assert_eq!(scope, expected)
    }

    #[test]
    fn query_sessions() {
        let bin = must_read_binary_file("testdata/scope.json");
        let scope: Scope = from_binary(&bin).unwrap();
        let bin = must_read_binary_file("testdata/sessions.json");
        let expected: Sessions = from_binary(&bin).unwrap();
        let querier = MetadataQuerier::new(scope, Some(expected.clone()), None);

        let params = MetadataQueryParams::GetSessions {
            scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let sessions: Sessions = from_binary(&bin).unwrap();

        assert_eq!(sessions, expected)
    }

    #[test]
    fn query_records() {
        let bin = must_read_binary_file("testdata/scope.json");
        let scope: Scope = from_binary(&bin).unwrap();
        let bin = must_read_binary_file("testdata/records.json");
        let expected: Records = from_binary(&bin).unwrap();
        let querier = MetadataQuerier::new(scope, None, Some(expected.clone()));

        let params = MetadataQueryParams::GetRecords {
            scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
            name: None,
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let records: Records = from_binary(&bin).unwrap();

        assert_eq!(records, expected)
    }

    #[test]
    fn query_records_by_name() {
        let bin = must_read_binary_file("testdata/scope.json");
        let scope: Scope = from_binary(&bin).unwrap();
        let bin = must_read_binary_file("testdata/records.json");
        let records: Records = from_binary(&bin).unwrap();
        let querier = MetadataQuerier::new(scope, None, Some(records.clone()));

        let bin = must_read_binary_file("testdata/loan_record.json");
        let expected: Records = from_binary(&bin).unwrap();

        let params = MetadataQueryParams::GetRecords {
            scope_id: "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d".into(),
            name: Some("loan".into()),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let records: Records = from_binary(&bin).unwrap();

        assert_eq!(records, expected)
    }
}
