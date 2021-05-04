use crate::common::{query_error, query_result};
use cosmwasm_std::{to_binary, Addr, QuerierResult};
use provwasm_std::{Marker, MarkerQueryParams};
use std::collections::HashMap;

/// A mock for testing provenance marker module queries.
#[derive(Clone, Default)]
pub struct MarkerQuerier {
    denom_records: HashMap<String, Marker>,
    address_records: HashMap<Addr, Marker>,
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

    fn get_marker_by_denom(&self, denom: &str) -> Option<QuerierResult> {
        self.denom_records
            .get(denom)
            .map(|marker| query_result(to_binary(&marker)))
    }

    fn get_marker_by_address(&self, address: &Addr) -> Option<QuerierResult> {
        self.address_records
            .get(address)
            .map(|marker| query_result(to_binary(&marker)))
    }

    pub fn query(&self, params: &MarkerQueryParams) -> QuerierResult {
        let maybe_marker = match params {
            MarkerQueryParams::GetMarkerByAddress { address } => {
                self.get_marker_by_address(address)
            }
            MarkerQueryParams::GetMarkerByDenom { denom } => self.get_marker_by_denom(denom),
        };
        match maybe_marker {
            Some(r) => r,
            None => query_error("marker not found", to_binary(params)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::must_read_binary_file;
    use cosmwasm_std::{from_binary, SystemError};
    use provwasm_std::{MarkerQueryParams, MarkerStatus, MarkerType};

    #[test]
    fn get_marker_by_denom() {
        let bin = must_read_binary_file("testdata/marker.json");
        let expected_marker: Marker = from_binary(&bin).unwrap();
        let querier = MarkerQuerier::new(vec![expected_marker.clone()]);

        let params = MarkerQueryParams::GetMarkerByDenom {
            denom: "nugz".into(),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let marker: Marker = from_binary(&bin).unwrap();

        assert_eq!(marker, expected_marker)
    }

    #[test]
    fn get_manager() {
        // Set up mocks
        let bin = must_read_binary_file("testdata/marker.json");
        let test_marker: Marker = from_binary(&bin).unwrap();
        let querier = MarkerQuerier::new(vec![test_marker]);

        // Query for marker
        let res = querier
            .query(&MarkerQueryParams::GetMarkerByDenom {
                denom: "nugz".into(),
            })
            .unwrap();
        let bin = res.unwrap();
        let marker: Marker = from_binary(&bin).unwrap();

        // Ensure the manager address is set on proposed marker.
        assert_eq!(marker.status, MarkerStatus::Proposed);
        let expected_manager = Addr::unchecked("tp15rrl3qjafxzlzguu5x29xh29pam35uetkpnnph");
        assert_eq!(marker.get_manager(), Some(expected_manager))
    }

    #[test]
    fn get_marker_by_address() {
        let bin = must_read_binary_file("testdata/marker.json");
        let expected_marker: Marker = from_binary(&bin).unwrap();
        let querier = MarkerQuerier::new(vec![expected_marker.clone()]);

        let params = MarkerQueryParams::GetMarkerByAddress {
            address: Addr::unchecked("tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u"),
        };
        let bin = querier.query(&params).unwrap().unwrap();
        let marker: Marker = from_binary(&bin).unwrap();

        assert_eq!(marker, expected_marker)
    }

    #[test]
    fn get_marker_not_found() {
        let querier = MarkerQuerier::default();
        let params = MarkerQueryParams::GetMarkerByDenom {
            denom: "budz".into(),
        };
        let err = querier.query(&params).unwrap_err();
        match err {
            SystemError::InvalidRequest { error, .. } => assert_eq!(error, "marker not found"),
            _ => panic!("unexpected error type"),
        }
    }

    #[test]
    fn get_active_restricted_marker() {
        // Assert that bank sends are disabled for restricted markers
        let bin = must_read_binary_file("testdata/marker_restricted.json");
        let marker: Marker = from_binary(&bin).unwrap();
        assert_eq!(marker.marker_type, MarkerType::Restricted);
        assert_eq!(marker.bank_sends_disabled(), true);
        // Check manager is None for active markers
        assert_eq!(marker.status, MarkerStatus::Active);
        assert_eq!(marker.get_manager(), None);
    }
}
