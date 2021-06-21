use crate::common::{query_error, query_result};
use cosmwasm_std::{to_binary, Addr, QuerierResult};
use provwasm_std::{MetadataQueryParams, Scope};
use std::collections::HashMap;

/// A mock for testing provenance metadata module queries.
#[derive(Clone, Default)]
pub struct MetadataQuerier {
    scope_records: HashMap<String, Scope>,
}

impl MetadataQuerier {
    pub fn new(inputs: Vec<Scope>) -> Self {
        let mut scope_records = HashMap::new();
        for s in inputs.into_iter() {
            scope_records.insert(s.scope_id.to_string(), s);
        }
        MetadataQuerier { scope_records }
    }

    fn get_scope(&self, scope_id: &Addr) -> Option<QuerierResult> {
        self.scope_records
            .get(&scope_id.to_string())
            .map(|scope| query_result(to_binary(scope)))
    }

    pub fn query(&self, params: &MetadataQueryParams) -> QuerierResult {
        match params {
            MetadataQueryParams::GetScope { scope_id } => match self.get_scope(scope_id) {
                Some(r) => r,
                None => query_error("scope not found", to_binary(params)),
            },
            MetadataQueryParams::GetSessions { .. } => todo!(),
            MetadataQueryParams::GetRecords { .. } => todo!(),
        }
    }
}
