use crate::{AttributeQuerier, MarkerQuerier, NameQuerier, ScopeQuerier};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    from_slice, to_binary, Coin, HumanAddr, OwnedDeps, Querier, QuerierResult, QueryRequest,
    StdResult, SystemError, SystemResult,
};
use provwasm_std::{Marker, ProvenanceQuery, ProvenanceQueryParams, Scope};

/// A drop-in replacement for cosmwasm_std::testing::mock_dependencies that uses the mock
/// provenance querier.
pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, ProvenanceMockQuerier> {
    let contract_addr = HumanAddr::from("provwasm2contract");
    let base = MockQuerier::new(&[(&contract_addr, contract_balance)]);
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: ProvenanceMockQuerier::new(base),
    }
}

pub struct ProvenanceMockQuerier {
    base: MockQuerier,
    name: NameQuerier,
    attribute: AttributeQuerier,
    scope: ScopeQuerier,
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
        match &request {
            QueryRequest::Custom(custom) => match &custom.params {
                ProvenanceQueryParams::Attribute(p) => self.attribute.query(&p),
                ProvenanceQueryParams::Name(p) => self.name.query(&p),
                ProvenanceQueryParams::Scope(p) => self.scope.query(&p),
                ProvenanceQueryParams::Marker(p) => self.marker.query(&p),
            },
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
            scope: ScopeQuerier::default(),
            marker: MarkerQuerier::default(),
        }
    }

    pub fn with_names(&mut self, inputs: &[(&str, &str, bool)]) {
        self.name = NameQuerier::new(inputs);
    }

    pub fn with_attributes(&mut self, address: &str, inputs: &[(&str, &str, &str)]) {
        self.attribute = AttributeQuerier::new(address, inputs);
    }

    pub fn with_scopes(&mut self, inputs: Vec<Scope>) {
        self.scope = ScopeQuerier::new(inputs);
    }

    pub fn with_markers(&mut self, inputs: Vec<Marker>) {
        self.marker = MarkerQuerier::new(inputs);
    }
}
