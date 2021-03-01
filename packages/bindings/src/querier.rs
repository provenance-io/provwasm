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
            params: ProvenanceQueryParams::Name(params),
            version: String::from(QUERY_DATAFMT_VERSION),
        };
        let res: Names = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    /// Resolve the address for a name.
    pub fn resolve_name<S: Into<String>>(&self, name: S) -> StdResult<Name> {
        let res = self.query_name_module(NameQueryParams::Resolve { name: name.into() })?;
        if res.records.len() != 1 {
            return Err(StdError::generic_err(
                "expected only one address bound to name",
            ));
        }
        Ok(res.records[0].clone())
    }

    /// Lookup all names bound to the given address.
    pub fn lookup_names<H: Into<HumanAddr>>(&self, address: H) -> StdResult<Names> {
        self.query_name_module(NameQueryParams::Lookup {
            address: address.into(),
        })
    }

    // Execute a attribute module query.
    fn query_attributes(&self, params: AttributeQueryParams) -> StdResult<Attributes> {
        let request = ProvenanceQuery {
            route: ProvenanceRoute::Attribute,
            params: ProvenanceQueryParams::Attribute(params),
            version: String::from(QUERY_DATAFMT_VERSION),
        };
        let res: Attributes = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    /// Get attributes for an account. If the name parameter is empty, all attributes are returned.
    pub fn get_attributes<H: Into<HumanAddr>, S: Into<String>>(
        &self,
        address: H,
        name: Option<S>,
    ) -> StdResult<Attributes> {
        match name {
            None => self.query_attributes(AttributeQueryParams::GetAllAttributes {
                address: address.into(),
            }),
            Some(name) => self.query_attributes(AttributeQueryParams::GetAttributes {
                address: address.into(),
                name: name.into(),
            }),
        }
    }

    /// Get named JSON attributes from an account and deserialize the values.
    /// Attribute values with the same name must be able to be deserialized to the same type.
    pub fn get_json_attributes<H: Into<HumanAddr>, S: Into<String>, T: DeserializeOwned>(
        &self,
        address: H,
        name: S,
    ) -> StdResult<Vec<T>> {
        // Gather results
        let resp = self.query_attributes(AttributeQueryParams::GetAttributes {
            address: address.into(),
            name: name.into(),
        })?;
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
            params: ProvenanceQueryParams::Marker(params),
            version: String::from(QUERY_DATAFMT_VERSION),
        };
        let res: Marker = self.querier.custom_query(&request.into())?;
        Ok(res)
    }

    /// Get a marker by address.
    pub fn get_marker_by_address<H: Into<HumanAddr>>(&self, address: H) -> StdResult<Marker> {
        self.query_marker(MarkerQueryParams::GetMarkerByAddress {
            address: address.into(),
        })
    }

    /// Get a marker by denomination.
    pub fn get_marker_by_denom<S: Into<String>>(&self, denom: S) -> StdResult<Marker> {
        self.query_marker(MarkerQueryParams::GetMarkerByDenom {
            denom: denom.into(),
        })
    }
}
