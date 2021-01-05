use cosmwasm_std::{
    to_binary, Binary, ContractResult, HumanAddr, QuerierResult, StdResult, SystemError,
    SystemResult,
};

use provwasm_std::{Marker, MarkerQueryParams};

use std::collections::HashMap;

/// A mock for testing provenance marker module queries.
#[derive(Clone, Default)]
pub struct MarkerQuerier {
    denom_records: HashMap<String, Marker>,
    address_records: HashMap<HumanAddr, Marker>,
}

impl MarkerQuerier {
    pub fn new(inputs: Vec<Marker>) -> Self {
        let mut denom_records = HashMap::new();
        let mut address_records = HashMap::new();
        for m in inputs.iter() {
            denom_records.insert(m.denom.clone(), m.clone());
            address_records.insert(m.address.clone(), m.clone());
        }
        MarkerQuerier {
            denom_records,
            address_records,
        }
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

    fn get_marker_by_denom(&self, denom: &str) -> Option<QuerierResult> {
        self.denom_records
            .get(denom)
            .map(|marker| self.query_result(to_binary(&marker)))
    }

    fn get_marker_by_address(&self, address: &HumanAddr) -> Option<QuerierResult> {
        self.address_records
            .get(address)
            .map(|marker| self.query_result(to_binary(&marker)))
    }

    pub fn query(&self, params: &MarkerQueryParams) -> QuerierResult {
        match params {
            MarkerQueryParams::GetMarker { denom, address } => {
                let maybe_marker = if denom.is_empty() {
                    self.get_marker_by_address(address)
                } else {
                    self.get_marker_by_denom(denom)
                };
                match maybe_marker {
                    Some(r) => r,
                    None => self.query_error("marker not found".into(), to_binary(params)),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::{from_binary, Binary};
    use provwasm_std::MarkerQueryParams;
    use std::fs::File;
    use std::io::Read;

    fn read_test_marker_from_file() -> Binary {
        let filename = "testdata/marker.json";
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
    fn get_marker_by_denom() {
        let bin = read_test_marker_from_file();
        let expected_marker: Marker = from_binary(&bin).unwrap();
        let querier = MarkerQuerier::new(vec![expected_marker.clone()]);

        let params = MarkerQueryParams::GetMarker {
            denom: "nugz".into(),
            address: "".into(),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let marker: Marker = from_binary(&bin).unwrap();

        assert_eq!(marker, expected_marker)
    }

    #[test]
    fn get_marker_by_address() {
        let bin = read_test_marker_from_file();
        let expected_marker: Marker = from_binary(&bin).unwrap();
        let querier = MarkerQuerier::new(vec![expected_marker.clone()]);

        let params = MarkerQueryParams::GetMarker {
            denom: "".into(),
            address: HumanAddr::from("tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u"),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let marker: Marker = from_binary(&bin).unwrap();

        assert_eq!(marker, expected_marker)
    }

    #[test]
    fn get_marker_not_found() {
        let querier = MarkerQuerier::default();
        let params = MarkerQueryParams::GetMarker {
            denom: "budz".into(),
            address: "".into(),
        };
        let err = querier.query(&params).unwrap_err();
        match err {
            SystemError::InvalidRequest { error, .. } => assert_eq!(error, "marker not found"),
            _ => panic!("unexpected error type"),
        }
    }
}
