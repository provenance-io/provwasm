use cosmwasm_std::{from_binary, HumanAddr, QuerierWrapper, StdError, StdResult};
use serde::de::DeserializeOwned;

use crate::query::*;
use crate::types::{AttributeValueType, Attributes, Marker, Name, Names, ProvenanceRoute};

// The data format version to pass into provenance for queries.
static QUERY_DATAFMT_VERSION: &str = "2.0.0";

/// A type for simplifying provenance custom queries.
pub struct ProvenanceQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

impl<'a> ProvenanceQuerier<'a> {
    /// Creates a new provenance querier
    pub fn new(querier: &'a QuerierWrapper) -> Self {
        ProvenanceQuerier { querier }
    }

    // Execute a name module query.
    fn query_name_module(&self, params: NameQueryParams) -> StdResult<Names> {
        let request = ProvenanceQuery {
            route: ProvenanceRoute::Name,
            params: params.into(),
            version: String::from(QUERY_DATAFMT_VERSION),
        };
        let res: Names = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    /// Resolve the address for a name.
    pub fn resolve_name(&self, name: String) -> StdResult<Name> {
        let res = self.query_name_module(NameQueryParams::Resolve { name })?;
        if res.records.len() != 1 {
            return Err(StdError::generic_err(
                "expected only one address bound to name",
            ));
        }
        Ok(res.records[0].clone())
    }

    /// Lookup all names bound to the given address.
    pub fn lookup_names(&self, address: HumanAddr) -> StdResult<Names> {
        self.query_name_module(NameQueryParams::Lookup { address })
    }

    // Execute a metadata module attribute query.
    fn query_attributes(&self, params: AttributeQueryParams) -> StdResult<Attributes> {
        let request = ProvenanceQuery {
            route: ProvenanceRoute::Attribute,
            params: params.into(),
            version: String::from(QUERY_DATAFMT_VERSION),
        };
        let res: Attributes = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    /// Get attributes for an account. If the name parameter is empty, all attributes are returned.
    pub fn get_attributes(
        &self,
        address: HumanAddr,
        name: Option<String>,
    ) -> StdResult<Attributes> {
        match name {
            None => self.query_attributes(AttributeQueryParams::GetAllAttributes { address }),
            Some(name) => {
                self.query_attributes(AttributeQueryParams::GetAttributes { address, name })
            }
        }
    }

    /// Get named JSON attributes from an account and deserialize the values.
    /// Attribute values with the same name must be able to be deserialized to the same type.
    pub fn get_json_attributes<T: DeserializeOwned>(
        &self,
        address: HumanAddr,
        name: String,
    ) -> StdResult<Vec<T>> {
        // Gather results
        let resp = self.query_attributes(AttributeQueryParams::GetAttributes { address, name })?;
        // Map deserialize, returning values or failure.
        resp.attributes
            .iter()
            .filter(|a| a.value_type == AttributeValueType::Json)
            .map(|a| from_binary(&a.value))
            .collect()
    }

    // Execute a marker module query.
    fn query_marker(&self, params: MarkerQueryParams) -> StdResult<Marker> {
        let request = ProvenanceQuery {
            route: ProvenanceRoute::Marker,
            params: params.into(),
            version: String::from(QUERY_DATAFMT_VERSION),
        };
        let res: Marker = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    /// Get a marker by address.
    pub fn get_marker_by_address(&self, address: HumanAddr) -> StdResult<Marker> {
        self.query_marker(MarkerQueryParams::GetMarkerByAddress { address })
    }

    /// Get a marker by denomination.
    pub fn get_marker_by_denom(&self, denom: String) -> StdResult<Marker> {
        self.query_marker(MarkerQueryParams::GetMarkerByDenom { denom })
    }
}
