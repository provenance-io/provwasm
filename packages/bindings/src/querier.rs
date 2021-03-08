use cosmwasm_std::{from_binary, HumanAddr, QuerierWrapper, StdError, StdResult};
use serde::de::DeserializeOwned;

use crate::query::*;
use crate::types::{Attributes, AttributeValueType, Marker, Name, Names, ProvenanceRoute};

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
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use cosmwasm_std::{Deps, QueryResponse, StdError};
    /// # use provwasm_std::{Name, ProvenanceQuerier};
    ///
    /// fn query_resolve_name(deps: Deps, name: String) -> Result<QueryResponse, StdError> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let name: Name = querier.resolve_name(&name)?;
    ///     // Do something with name...
    ///     todo!()
    /// }
    /// ```
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
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use cosmwasm_std::{Deps, HumanAddr, QueryResponse, StdError};
    /// # use provwasm_std::{Names, ProvenanceQuerier};
    ///
    /// fn query_lookup_names(deps: Deps, address: HumanAddr) -> Result<QueryResponse, StdError> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let names: Names = querier.lookup_names(&address)?;
    ///     // Do something with names...
    ///     todo!()
    /// }
    /// ```
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

    /// Get attributes for an account. If the name parameter is `None`, all attributes are returned.
    ///
    ///  ### Example
    ///
    /// ```rust
    /// # use cosmwasm_std::{Deps, HumanAddr, QueryResponse, StdError};
    /// # use provwasm_std::{Attributes, ProvenanceQuerier};
    ///
    /// // Query all label attributes added to an account.
    /// pub fn try_query_attributes(deps: Deps, address: HumanAddr) -> Result<QueryResponse, StdError> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let none: Option<String> = None;
    ///     let attrs: Attributes = querier.get_attributes(&address, none)?;
    ///     // Do something with attrs...
    ///     todo!()
    /// }
    /// ```
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
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use cosmwasm_std::{Deps, HumanAddr, QueryResponse, StdError};
    /// # use provwasm_std::ProvenanceQuerier;
    /// # use schemars::JsonSchema;
    /// # use serde::{Deserialize, Serialize};
    ///
    /// // Query all label attributes added to an account.
    /// pub fn query_labels(deps: Deps, address: HumanAddr) -> Result<QueryResponse, StdError> {
    ///     let attr_name = String::from("label.my-contract.sc.pb");
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let labels: Vec<Label> = querier.get_json_attributes(&address, &attr_name)?;
    ///     // Do something with labels...
    ///     todo!()
    /// }
    ///
    /// // Text with timestamp.
    /// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    /// #[serde(rename_all = "snake_case")]
    /// pub struct Label {
    ///     pub text: String,
    ///     pub timestamp: u64,
    /// }
    /// ```
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
    ///
    /// ### Example
    ///
    /// ```rust
    /// // Query a marker by address.
    /// use provwasm_std::{ProvenanceQuerier, Marker};
    /// use cosmwasm_std::{Deps, HumanAddr, QueryResponse, StdError, to_binary};
    /// fn try_get_marker_by_address(deps: Deps, address: HumanAddr) -> Result<QueryResponse, StdError> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let marker: Marker = querier.get_marker_by_address(&address)?;
    ///     to_binary(&marker)
    /// }
    /// ```
    pub fn get_marker_by_address<H: Into<HumanAddr>>(&self, address: H) -> StdResult<Marker> {
        self.query_marker(MarkerQueryParams::GetMarkerByAddress {
            address: address.into(),
        })
    }

    /// Get a marker by denomination.
    ///
    /// ### Example
    ///
    /// To query a marker by denomination
    ///
    /// ```rust
    /// // Query a marker by denom.
    /// use cosmwasm_std::{Deps, QueryResponse, StdError, to_binary};
    /// use provwasm_std::{ProvenanceQuerier, Marker};
    /// fn try_get_marker_by_denom(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let marker: Marker = querier.get_marker_by_denom(&denom)?;
    ///     to_binary(&marker)
    /// }
    /// ```
    pub fn get_marker_by_denom<S: Into<String>>(&self, denom: S) -> StdResult<Marker> {
        self.query_marker(MarkerQueryParams::GetMarkerByDenom {
            denom: denom.into(),
        })
    }
}
