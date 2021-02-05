use crate::common::{query_error, query_result};
use cosmwasm_std::{to_binary, QuerierResult};
use provwasm_std::{Scope, ScopeQueryParams};
use std::collections::HashMap;

/// A mock for testing provenance metadata module queries.
#[derive(Clone, Default)]
pub struct ScopeQuerier {
    records: HashMap<String, Scope>,
}

/// Implements mock scope query behavior.
impl ScopeQuerier {
    pub fn new(inputs: Vec<Scope>) -> Self {
        let mut records = HashMap::new();
        for s in inputs.iter() {
            records.insert(s.id.clone(), s.clone());
        }
        ScopeQuerier { records }
    }

    pub fn query(&self, params: &ScopeQueryParams) -> QuerierResult {
        match params {
            ScopeQueryParams::GetScope { id } => {
                let maybe_scope = self
                    .records
                    .get(id)
                    .map(|scope| query_result(to_binary(&scope)));
                match maybe_scope {
                    Some(r) => r,
                    None => query_error(
                        &format!("scope does not exist for uuid {}", id),
                        to_binary(params),
                    ),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::must_read_binary_file;
    use cosmwasm_std::{from_binary, SystemError};
    use provwasm_std::ScopeQueryParams;

    #[test]
    fn get_scope() {
        let bin = must_read_binary_file("testdata/scope.json");
        let expected_scope: Scope = from_binary(&bin).unwrap();
        let querier = ScopeQuerier::new(vec![expected_scope.clone()]);

        let params = ScopeQueryParams::GetScope {
            id: "fbd81e76-fb4b-44f4-98dd-96f78b654f47".into(),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let scope: Scope = from_binary(&bin).unwrap();

        assert_eq!(scope, expected_scope)
    }

    #[test]
    fn get_scope_not_found() {
        let querier = ScopeQuerier::default();
        let params = ScopeQueryParams::GetScope {
            id: "fbd81e76-fb4b-44f4-98dd-96f78b654f47".into(),
        };
        let err = querier.query(&params).unwrap_err();
        match err {
            SystemError::InvalidRequest { error, .. } => assert_eq!(
                error,
                "scope does not exist for uuid fbd81e76-fb4b-44f4-98dd-96f78b654f47"
            ),
            _ => panic!("unexpected error type"),
        }
    }
}
