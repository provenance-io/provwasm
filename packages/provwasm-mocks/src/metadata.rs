use crate::common::{query_error, query_result};
use cosmwasm_std::{to_binary, QuerierResult};
use provwasm_std::{MetadataQueryParams, Records, Scope, Sessions};
use std::collections::HashMap;

/// A mock for testing provenance metadata module queries.
#[deprecated(since = "2.0.0")]
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
    use cosmwasm_std::{from_binary, Binary};

    // The scope ID used for test queries.
    const TEST_SCOPE_ID: &str = "scope1qqqqq2wf3c4yt4u447m8pw65qcdqrre82d";

    // Read a test scope from file.
    fn test_scope() -> Scope {
        let bin = must_read_binary_file("testdata/scope.json");
        from_binary(&bin).unwrap()
    }
    // Read test sessions from file.
    fn test_sessions() -> Sessions {
        let bin = must_read_binary_file("testdata/sessions.json");
        from_binary(&bin).unwrap()
    }

    // Read test records from file.
    fn test_records() -> Records {
        let bin = must_read_binary_file("testdata/records.json");
        from_binary(&bin).unwrap()
    }

    // Execute a query against the mock metadata querier.
    fn mock_query(params: &MetadataQueryParams) -> Binary {
        let querier =
            MetadataQuerier::new(test_scope(), Some(test_sessions()), Some(test_records()));
        querier.query(params).unwrap().unwrap()
    }

    #[test]
    fn query_scope() {
        let result = mock_query(&MetadataQueryParams::GetScope {
            scope_id: TEST_SCOPE_ID.into(),
        });
        let scope: Scope = from_binary(&result).unwrap();
        assert_eq!(scope, test_scope())
    }

    #[test]
    fn query_sessions() {
        let result = mock_query(&MetadataQueryParams::GetSessions {
            scope_id: TEST_SCOPE_ID.into(),
        });
        let sessions: Sessions = from_binary(&result).unwrap();
        assert_eq!(sessions, test_sessions())
    }

    #[test]
    fn query_records() {
        let result = mock_query(&MetadataQueryParams::GetRecords {
            scope_id: TEST_SCOPE_ID.into(),
            name: None,
        });
        let records: Records = from_binary(&result).unwrap();
        assert_eq!(records, test_records())
    }

    #[test]
    fn query_records_by_name() {
        let bin = must_read_binary_file("testdata/loan_record.json");
        let expected: Records = from_binary(&bin).unwrap();

        let result = mock_query(&MetadataQueryParams::GetRecords {
            scope_id: TEST_SCOPE_ID.into(),
            name: Some("loan".into()),
        });

        let records: Records = from_binary(&result).unwrap();
        assert_eq!(records, expected)
    }
}
