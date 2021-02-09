#![allow(clippy::field_reassign_with_default)]
use cosmwasm_std::{CustomQuery, HumanAddr};
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
    Lookup { address: HumanAddr },
}

/// A helper that wraps NameQueryParams in ProvenanceQueryParams.
impl From<NameQueryParams> for ProvenanceQueryParams {
    fn from(params: NameQueryParams) -> ProvenanceQueryParams {
        ProvenanceQueryParams::Name(params)
    }
}

/// Params for attribute queries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeQueryParams {
    GetAttributes { address: HumanAddr, name: String },
    GetAllAttributes { address: HumanAddr },
}

/// A helper that wraps AttributeQueryParams in ProvenanceQueryParams.
impl From<AttributeQueryParams> for ProvenanceQueryParams {
    fn from(params: AttributeQueryParams) -> ProvenanceQueryParams {
        ProvenanceQueryParams::Attribute(params)
    }
}

/// Params for marker queries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkerQueryParams {
    GetMarkerByAddress { address: HumanAddr },
    GetMarkerByDenom { denom: String },
}

/// A helper that wraps MarkerQueryParams in ProvenanceQueryParams.
impl From<MarkerQueryParams> for ProvenanceQueryParams {
    fn from(params: MarkerQueryParams) -> ProvenanceQueryParams {
        ProvenanceQueryParams::Marker(params)
    }
}
