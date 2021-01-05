use cosmwasm_std::{
    to_binary, Binary, ContractResult, QuerierResult, StdResult, SystemError, SystemResult,
};

use provwasm_std::{Scope, ScopeQueryParams};

use std::collections::HashMap;

/// A mock for testing provenance metadata module queries.
#[derive(Clone, Default)]
pub struct ScopeQuerier {
    records: HashMap<String, Scope>,
}

impl ScopeQuerier {
    pub fn new(inputs: Vec<Scope>) -> Self {
        let mut records = HashMap::new();
        for s in inputs.iter() {
            let scope = s.clone();
            records.insert(s.id.clone(), scope);
        }
        ScopeQuerier { records }
    }

    fn query_result(&self, bin: StdResult<Binary>) -> QuerierResult {
        SystemResult::Ok(ContractResult::Ok(bin.unwrap()))
    }

    fn query_error(&self, error: String, bin: StdResult<Binary>) -> QuerierResult {
        SystemResult::Err(SystemError::InvalidRequest {
            error,
            request: bin.unwrap(),
        })
    }

    pub fn query(&self, params: &ScopeQueryParams) -> QuerierResult {
        match params {
            ScopeQueryParams::GetScope { id } => {
                let maybe_scope = self
                    .records
                    .get(id)
                    .map(|scope| self.query_result(to_binary(&scope)));
                match maybe_scope {
                    Some(r) => r,
                    None => self.query_error(
                        format!("scope does not exist for uuid {}", id),
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
    use cosmwasm_std::{from_binary, Binary};
    use provwasm_std::ScopeQueryParams;
    use std::fs::File;
    use std::io::Read;

    fn read_test_scope_from_file() -> Binary {
        let filename = "testdata/scope.json";
        match File::open(filename) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                Binary::from(content.as_bytes())
            }
            Err(error) => {
                panic!("Error opening file {}: {}", filename, error);
            }
        }
    }

    #[test]
    fn get_scope() {
        let bin = read_test_scope_from_file();
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
