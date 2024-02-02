use std::collections::HashMap;
use std::marker::PhantomData;

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    from_json, Binary, Coin, Empty, OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError,
    SystemResult,
};
use serde::de::DeserializeOwned;

use provwasm_common::MockableQuerier;

pub struct MockProvenanceQuerier<C: DeserializeOwned = Empty> {
    /// Default CosmWASM mock querier.
    pub mock_querier: MockQuerier<C>,
    /// Registered custom queries using proto request for testing.
    #[allow(clippy::complexity)]
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
        let request: QueryRequest<Empty> = match from_json(bin_request) {
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
    use cosmwasm_std::{coin, from_binary, BalanceResponse, BankQuery};

    use super::*;

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
