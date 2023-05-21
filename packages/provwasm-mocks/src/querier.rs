use crate::{AttributeQuerier, MarkerQuerier, MetadataQuerier, NameQuerier};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_slice, to_binary, Binary, Coin, Empty, OwnedDeps, Querier, QuerierResult, QueryRequest,
    StdResult, SystemError, SystemResult,
};
use provwasm_common::MockableQuerier;
use provwasm_std::{Marker, ProvenanceQuery, ProvenanceQueryParams, Records, Scope, Sessions};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::marker::PhantomData;

/// A drop-in replacement for cosmwasm_std::testing::mock_dependencies that uses the mock
/// provenance querier.
#[deprecated(since = "2.0.0")]
pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, ProvenanceMockQuerier, ProvenanceQuery> {
    let base = MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]);
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: ProvenanceMockQuerier::new(base),
        custom_query_type: PhantomData,
    }
}

/// Initializes the mock querier with the account balances provided. NOTE: contract balance must
/// be set in the balances slice passed if required.
#[deprecated(since = "2.0.0")]
pub fn mock_dependencies_with_balances(
    balances: &[(&str, &[Coin])],
) -> OwnedDeps<MockStorage, MockApi, ProvenanceMockQuerier, ProvenanceQuery> {
    let base = MockQuerier::new(balances);
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: ProvenanceMockQuerier::new(base),
        custom_query_type: PhantomData,
    }
}

#[deprecated(since = "2.0.0")]
pub struct ProvenanceMockQuerier {
    pub base: MockQuerier,
    name: NameQuerier,
    attribute: AttributeQuerier,
    marker: MarkerQuerier,
    metadata: MetadataQuerier,
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
                ProvenanceQueryParams::Attribute(p) => self.attribute.query(p),
                ProvenanceQueryParams::Marker(p) => self.marker.query(p),
                ProvenanceQueryParams::Name(p) => self.name.query(p),
                ProvenanceQueryParams::Metadata(p) => self.metadata.query(p),
            },
            QueryRequest::Bank(q) => self.base.handle_query(&QueryRequest::Bank(q.clone())),
            #[cfg(feature = "staking")]
            QueryRequest::Staking(q) => self.base.handle_query(&QueryRequest::Staking(q.clone())),
            QueryRequest::Wasm(q) => self.base.handle_query(&QueryRequest::Wasm(q.clone())),
            // Note: As of 0.14, no mocks exist for stargate or ibc queries in base, so we just
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
            metadata: MetadataQuerier::default(),
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

    pub fn with_scope(&mut self, scope: Scope) {
        self.metadata = MetadataQuerier::new(scope, None, None)
    }

    pub fn with_sessions(&mut self, scope: Scope, sessions: Sessions) {
        self.metadata = MetadataQuerier::new(scope, Some(sessions), None)
    }

    pub fn with_records(&mut self, scope: Scope, records: Records) {
        self.metadata = MetadataQuerier::new(scope, None, Some(records))
    }

    pub fn with_metadata(&mut self, scope: Scope, sessions: Sessions, records: Records) {
        self.metadata = MetadataQuerier::new(scope, Some(sessions), Some(records))
    }
}

pub struct MockProvenanceQuerier<C: DeserializeOwned = Empty> {
    /// Default CosmWASM mock querier.
    pub mock_querier: MockQuerier<C>,
    /// Registered custom queries using proto request for testing.
    pub registered_custom_queries: HashMap<String, Box<dyn Fn(&Binary) -> QuerierResult>>,
}

impl MockProvenanceQuerier {
    /// Initialize a new [`MockProvenanceQuerier`].
    /// * `balances` - Slice of balances passed to the `bank` module, where the first element
    /// is the user address and the second element is the user's balance.
    pub fn new(balances: &[(&str, &[Coin])]) -> Self {
        MockProvenanceQuerier {
            mock_querier: MockQuerier::new(balances),
            registered_custom_queries: HashMap::new(),
        }
    }
    /// Handle the query request.
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match request {
            QueryRequest::Stargate { path, data } => {
                if let Some(response_fn) = self.registered_custom_queries.get(path) {
                    return response_fn(data);
                }
                SystemResult::Err(SystemError::UnsupportedRequest { kind: path.into() })
            }
            _ => self.mock_querier.handle_query(request),
        }
    }
}

impl MockableQuerier for MockProvenanceQuerier {
    fn register_custom_query(
        &mut self,
        path: String,
        response_fn: Box<dyn Fn(&Binary) -> QuerierResult>,
    ) {
        self.registered_custom_queries.insert(path, response_fn);
    }
}

impl Querier for MockProvenanceQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl Default for MockProvenanceQuerier {
    fn default() -> Self {
        MockProvenanceQuerier::new(&[])
    }
}

/// Creates an instance of [`OwnedDeps`](cosmwasm_std::OwnedDeps) with a custom [`MockProvenanceQuerier`]
/// to allow the user to mock the query responses of one or more Provenance's modules.
pub fn mock_provenance_dependencies_with_custom_querier(
    querier: MockProvenanceQuerier,
) -> OwnedDeps<MockStorage, MockApi, MockProvenanceQuerier, Empty> {
    OwnedDeps::<_, _, _, Empty> {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier,
        custom_query_type: PhantomData,
    }
}

/// Creates an instance of [`OwnedDeps`](cosmwasm_std::OwnedDeps) that is capable of
/// handling queries towards Provenance's modules.
pub fn mock_provenance_dependencies(
) -> OwnedDeps<MockStorage, MockApi, MockProvenanceQuerier, Empty> {
    mock_provenance_dependencies_with_custom_querier(MockProvenanceQuerier::default())
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

    #[test]
    fn query_with_balances_2_0() {
        let amount = coin(12345, "denom");
        let querier = MockProvenanceQuerier::new(&[("alice", &[amount.clone()])]);
        let deps = mock_provenance_dependencies_with_custom_querier(querier);
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
