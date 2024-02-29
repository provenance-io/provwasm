use cosmwasm_std::{
    testing::{mock_env, MockApi, MockStorage},
    to_json_binary, Addr, Binary, Coin, ContractResult, DepsMut, Empty, MessageInfo, OwnedDeps,
    SystemResult,
};
use provwasm_mocks::MockProvenanceQuerier;
use provwasm_std::{
    shim::Any,
    types::provenance::{
        marker::v1::{QueryMarkerRequest, QueryMarkerResponse},
        metadata::v1::{ScopeRequest, ScopeResponse, ScopeWrapper},
    },
};

use crate::contract;

use super::{
    constants::{CREATOR, TEST_AMOUNT, TEST_DENOM},
    msg::mock_instantiate_msg,
};

pub fn mock_info(funds: bool, sender: &str) -> MessageInfo {
    if funds {
        return MessageInfo {
            sender: Addr::unchecked(sender),
            funds: vec![Coin::new(TEST_AMOUNT, TEST_DENOM)],
        };
    }
    return MessageInfo {
        sender: Addr::unchecked(sender),
        funds: vec![],
    };
}

pub fn mock_contract(deps: DepsMut) {
    let info = mock_info(false, CREATOR);
    let env = mock_env();
    let msg = mock_instantiate_msg();
    contract::instantiate(deps, env, info, msg).unwrap();
}

pub fn mock_scopes(deps: &mut OwnedDeps<MockStorage, MockApi, MockProvenanceQuerier, Empty>) {
    let path = "/provenance.metadata.v1.Query/Scope";
    let cb = Box::new(|bin: &Binary| -> SystemResult<ContractResult<Binary>> {
        let message = ScopeRequest::try_from(bin.clone()).unwrap();
        let mut response = ScopeResponse {
            scope: None,
            sessions: vec![],
            records: vec![],
            request: None,
        };
        if message.scope_id == "scope".to_string() {
            response = ScopeResponse {
                scope: Some(ScopeWrapper::default()),
                sessions: vec![],
                records: vec![],
                request: None,
            };
        }
        let binary = to_json_binary(&response).unwrap();
        SystemResult::Ok(ContractResult::Ok(binary))
    });

    deps.querier
        .registered_custom_queries
        .insert(path.to_string(), cb);
}

pub fn mock_markers(deps: &mut OwnedDeps<MockStorage, MockApi, MockProvenanceQuerier, Empty>) {
    let path = "/provenance.marker.v1.Query/Marker";
    let cb = Box::new(|bin: &Binary| -> SystemResult<ContractResult<Binary>> {
        let message = QueryMarkerRequest::try_from(bin.clone()).unwrap();
        let mut response = QueryMarkerResponse { marker: None };
        if message.id == "marker".to_string() {
            response = QueryMarkerResponse {
                marker: Some(Any {
                    type_url: "/provenance.marker.v1.MarkerAccount".to_string(),
                    value: vec![],
                }),
            };
        }
        let binary = to_json_binary(&response).unwrap();
        SystemResult::Ok(ContractResult::Ok(binary))
    });

    deps.querier
        .registered_custom_queries
        .insert(path.to_string(), cb);
}
