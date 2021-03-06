use cosmwasm_std::{from_binary, Addr, QuerierWrapper, StdError, StdResult};
use serde::de::DeserializeOwned;

use crate::common::{validate_address, validate_string};
use crate::query::{
    AttributeQueryParams, MarkerQueryParams, NameQueryParams, ProvenanceQuery,
    ProvenanceQueryParams,
};
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
    ///
    /// ### Example
    ///
    /// ```rust
    /// // Imports required
    /// use cosmwasm_std::{Deps, QueryResponse, StdResult};
    /// use provwasm_std::{Name, ProvenanceQuerier};
    ///
    /// // Resolve the address for a name.
    /// fn query_resolve_name(deps: Deps, name: String) -> StdResult<QueryResponse> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let name: Name = querier.resolve_name(&name)?;
    ///     // Do something with name.address ...
    ///     todo!()
    /// }
    /// ```
    pub fn resolve_name<S: Into<String>>(&self, name: S) -> StdResult<Name> {
        let res = self.query_name_module(NameQueryParams::Resolve {
            name: validate_string(name, "name")?,
        })?;
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
    /// // Imports required
    /// use cosmwasm_std::{Addr, Deps, QueryResponse, StdResult};
    /// use provwasm_std::{Names, ProvenanceQuerier};
    ///
    /// // Lookup all names bound to an address.
    /// fn query_lookup_names(deps: Deps, address: Addr) -> StdResult<QueryResponse> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let names: Names = querier.lookup_names(address)?;
    ///     // Do something with names.records ...
    ///     todo!()
    /// }
    /// ```
    pub fn lookup_names<H: Into<Addr>>(&self, address: H) -> StdResult<Names> {
        self.query_name_module(NameQueryParams::Lookup {
            address: validate_address(address)?,
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
    /// // Imports required
    /// use cosmwasm_std::{Addr, Deps, QueryResponse, StdResult};
    /// use provwasm_std::{Attributes, ProvenanceQuerier};
    ///
    /// // Query all attributes added to an account.
    /// pub fn try_query_attributes(deps: Deps, address: Addr) -> StdResult<QueryResponse> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let none: Option<String> = None;
    ///     let res: Attributes = querier.get_attributes(address, none)?;
    ///     // Do something with res.attributes ...
    ///     todo!()
    /// }
    /// ```
    pub fn get_attributes<H: Into<Addr>, S: Into<String>>(
        &self,
        address: H,
        name: Option<S>,
    ) -> StdResult<Attributes> {
        let address = validate_address(address)?;
        match name {
            None => self.query_attributes(AttributeQueryParams::GetAllAttributes { address }),
            Some(name) => self.query_attributes(AttributeQueryParams::GetAttributes {
                address,
                name: validate_string(name, "name")?,
            }),
        }
    }

    /// Get named JSON attributes from an account and deserialize the values.
    /// Attribute values with the same name must be able to be deserialized to the same type.
    ///
    /// ### Example
    ///
    /// ```rust
    /// // Imports required
    /// use cosmwasm_std::{Addr, Deps, QueryResponse, StdResult};
    /// use provwasm_std::ProvenanceQuerier;
    /// use schemars::JsonSchema;
    /// use serde::{Deserialize, Serialize};
    ///
    /// // Query all label attributes added to an account.
    /// pub fn query_labels(deps: Deps, address: Addr) -> StdResult<QueryResponse> {
    ///     let attr_name = String::from("label.my-contract.sc.pb");
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let labels: Vec<Label> = querier.get_json_attributes(address, &attr_name)?;
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
    pub fn get_json_attributes<H: Into<Addr>, S: Into<String>, T: DeserializeOwned>(
        &self,
        address: H,
        name: S,
    ) -> StdResult<Vec<T>> {
        // Gather results
        let resp = self.query_attributes(AttributeQueryParams::GetAttributes {
            address: validate_address(address)?,
            name: validate_string(name, "name")?,
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
    /// // Imports required
    /// use provwasm_std::{ProvenanceQuerier, Marker};
    /// use cosmwasm_std::{Addr, Deps, QueryResponse, StdResult};
    ///
    /// // Query a marker by address.
    /// fn try_get_marker_by_address(deps: Deps, address: Addr) -> StdResult<QueryResponse> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let marker: Marker = querier.get_marker_by_address(address)?;
    ///     // Do something with marker ...
    ///     todo!()
    /// }
    /// ```
    pub fn get_marker_by_address<H: Into<Addr>>(&self, address: H) -> StdResult<Marker> {
        self.query_marker(MarkerQueryParams::GetMarkerByAddress {
            address: validate_address(address)?,
        })
    }

    /// Get a marker by denomination.
    ///
    /// ### Example
    ///
    /// ```rust
    /// // Imports required
    /// use cosmwasm_std::{Deps, QueryResponse, StdResult};
    /// use provwasm_std::{ProvenanceQuerier, Marker};
    ///
    /// // Query a marker by denom.
    /// fn try_get_marker_by_denom(deps: Deps, denom: String) -> StdResult<QueryResponse> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let marker: Marker = querier.get_marker_by_denom(&denom)?;
    ///     // Do something with marker ...
    ///     todo!()
    /// }
    /// ```
    pub fn get_marker_by_denom<S: Into<String>>(&self, denom: S) -> StdResult<Marker> {
        self.query_marker(MarkerQueryParams::GetMarkerByDenom {
            denom: validate_string(denom, "denom")?,
        })
    }
}
