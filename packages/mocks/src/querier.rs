use crate::{AttributeQuerier, MarkerQuerier, NameQuerier};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_slice, to_binary, Coin, OwnedDeps, Querier, QuerierResult, QueryRequest, StdResult,
    SystemError, SystemResult,
};
use provwasm_std::{Marker, ProvenanceQuery, ProvenanceQueryParams};

/// A drop-in replacement for cosmwasm_std::testing::mock_dependencies that uses the mock
/// provenance querier.
pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, ProvenanceMockQuerier> {
    let base = MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]);
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: ProvenanceMockQuerier::new(base),
    }
}

/// Initializes the mock querier with the account balances provided. NOTE: contract balance must
/// be set in the balances slice passed if required.
pub fn mock_dependencies_with_balances(
    balances: &[(&str, &[Coin])],
) -> OwnedDeps<MockStorage, MockApi, ProvenanceMockQuerier> {
    let base = MockQuerier::new(balances);
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: ProvenanceMockQuerier::new(base),
    }
}

pub struct ProvenanceMockQuerier {
    pub base: MockQuerier,
    name: NameQuerier,
    attribute: AttributeQuerier,
    marker: MarkerQuerier,
}

impl Querier for ProvenanceMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: StdResult<QueryRequest<ProvenanceQuery>> = from_slice(bin_request);
        match &request {
            Ok(provenance_query) => self.handle_query(provenance_query),
            _ => self.base.raw_query(bin_request),
        }
    }
}

impl ProvenanceMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<ProvenanceQuery>) -> QuerierResult {
        match request {
            QueryRequest::Custom(custom) => match &custom.params {
                ProvenanceQueryParams::Attribute(p) => self.attribute.query(&p),
                ProvenanceQueryParams::Marker(p) => self.marker.query(&p),
                ProvenanceQueryParams::Name(p) => self.name.query(&p),
            },
            QueryRequest::Bank(q) => self.base.handle_query(&QueryRequest::Bank(q.clone())),
            QueryRequest::Staking(q) => self.base.handle_query(&QueryRequest::Staking(q.clone())),
            QueryRequest::Wasm(q) => self.base.handle_query(&QueryRequest::Wasm(q.clone())),
            // Note: As of 0.14.0-beta1, no mocking for stargate or ibc queries in base, so we just
            // return an error.
            _ => SystemResult::Err(SystemError::InvalidRequest {
                error: "invalid query request type".into(),
                request: to_binary(&request).unwrap(),
            }),
        }
    }
}

impl ProvenanceMockQuerier {
    pub fn new(base: MockQuerier) -> Self {
        ProvenanceMockQuerier {
            base,
            attribute: AttributeQuerier::default(),
            name: NameQuerier::default(),
            marker: MarkerQuerier::default(),
        }
    }

    pub fn with_names(&mut self, inputs: &[(&str, &str, bool)]) {
        self.name = NameQuerier::new(inputs);
    }

    pub fn with_attributes(&mut self, address: &str, inputs: &[(&str, &str, &str)]) {
        self.attribute = AttributeQuerier::new(address, inputs);
    }

    pub fn with_markers(&mut self, inputs: Vec<Marker>) {
        self.marker = MarkerQuerier::new(inputs);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::{coin, from_binary, BalanceResponse, BankQuery};

    #[test]
    fn query_with_balances() {
        let amount = coin(12345, "denom");
        let deps = mock_dependencies_with_balances(&[("alice", &[amount.clone()])]);
        let bin = deps
            .querier
            .handle_query(
                &BankQuery::Balance {
                    address: "alice".into(),
                    denom: "denom".into(),
                }
                .into(),
            )
            .unwrap()
            .unwrap();

        let res: BalanceResponse = from_binary(&bin).unwrap();
        assert_eq!(res.amount, amount);
    }
}
