use cosmwasm_std::{Addr, CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::ProvenanceRoute;

/// Represents a request to query a custom provenance module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProvenanceQuery {
    pub route: ProvenanceRoute,        // The module router key
    pub params: ProvenanceQueryParams, // The module-specific params
    pub version: String,               // The data format version
}

// Indicate that ProvenanceQuery is a custom query to avoid conflicts with other types such as
// BankQuery and WasmQuery.
impl CustomQuery for ProvenanceQuery {}

/// Input params for custom provenance queriers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceQueryParams {
    Name(NameQueryParams),
    Attribute(AttributeQueryParams),
    Marker(MarkerQueryParams),
}

/// Params for name queries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NameQueryParams {
    Resolve { name: String },
    Lookup { address: Addr },
}

/// Params for attribute queries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeQueryParams {
    GetAttributes { address: Addr, name: String },
    GetAllAttributes { address: Addr },
}

/// Params for marker queries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkerQueryParams {
    GetMarkerByAddress { address: Addr },
    GetMarkerByDenom { denom: String },
}
