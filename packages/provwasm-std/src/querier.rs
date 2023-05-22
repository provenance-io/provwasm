use cosmwasm_std::{from_binary, Addr, QuerierWrapper, StdError, StdResult};
use serde::de::DeserializeOwned;

use crate::common::{validate_address, validate_string};
use crate::query::{
    AttributeQueryParams, MarkerQueryParams, MetadataQueryParams, NameQueryParams, ProvenanceQuery,
    ProvenanceQueryParams,
};
use crate::types::{
    AttributeValueType, Attributes, Marker, Name, Names, ProvenanceRoute, Record, Records, Scope,
    Sessions,
};

// The data format version to pass into provenance for queries.
static QUERY_DATAFMT_VERSION: &str = "2.0.0";

/// A type for simplifying provenance custom queries.
#[deprecated(since = "2.0.0")]
pub struct ProvenanceQuerier<'a> {
    querier: &'a QuerierWrapper<'a, ProvenanceQuery>,
}

impl<'a> ProvenanceQuerier<'a> {
    /// Creates a new provenance querier
    pub fn new(querier: &'a QuerierWrapper<'a, ProvenanceQuery>) -> Self {
        ProvenanceQuerier { querier }
    }

    // Execute a name module query.
    fn query_name_module(&self, params: NameQueryParams) -> StdResult<Names> {
        let request = ProvenanceQuery {
            route: ProvenanceRoute::Name,
            params: ProvenanceQueryParams::Name(params),
            version: String::from(QUERY_DATAFMT_VERSION),
        };
        let res: Names = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Resolve the address for a name.
    ///
    /// ### Example
    ///
    /// ```rust
    /// // Imports required
    /// use cosmwasm_std::{Deps, QueryResponse, StdResult};
    /// use provwasm_std::{Name, ProvenanceQuerier, ProvenanceQuery};
    ///
    /// // Resolve the address for a name.
    /// fn query_resolve_name(deps: Deps<ProvenanceQuery>, name: String) -> StdResult<QueryResponse> {
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
    /// use provwasm_std::{Names, ProvenanceQuerier, ProvenanceQuery};
    ///
    /// // Lookup all names bound to an address.
    /// fn query_lookup_names(deps: Deps<ProvenanceQuery>, address: Addr) -> StdResult<QueryResponse> {
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
        let res: Attributes = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Get attributes for an account. If the name parameter is `None`, all attributes are returned.
    ///
    ///  ### Example
    ///
    /// ```rust
    /// // Imports required
    /// use cosmwasm_std::{Addr, Deps, QueryResponse, StdResult};
    /// use provwasm_std::{Attributes, ProvenanceQuerier, ProvenanceQuery};
    ///
    /// // Query all attributes added to an account.
    /// pub fn try_query_attributes(deps: Deps<ProvenanceQuery>, address: Addr) -> StdResult<QueryResponse> {
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
    /// use provwasm_std::{ProvenanceQuerier, ProvenanceQuery};
    /// use schemars::JsonSchema;
    /// use serde::{Deserialize, Serialize};
    ///
    /// // Query all label attributes added to an account.
    /// pub fn query_labels(deps: Deps<ProvenanceQuery>, address: Addr) -> StdResult<QueryResponse> {
    ///     let attr_name = String::from("label.my-contract.sc.pb");
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let labels: Vec<Label> = querier.get_json_attributes(address, &attr_name)?;
    ///     // Do something with labels...
    ///     todo!()
    /// }
    ///
    /// // Text with timestamp.
    /// #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
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
        let res: Marker = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Get a marker by address.
    ///
    /// ### Example
    ///
    /// ```rust
    /// // Imports required
    /// use provwasm_std::{ProvenanceQuerier, Marker, ProvenanceQuery};
    /// use cosmwasm_std::{Addr, Deps, QueryResponse, StdResult};
    ///
    /// // Query a marker by address.
    /// fn try_get_marker_by_address(deps: Deps<ProvenanceQuery>, address: Addr) -> StdResult<QueryResponse> {
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
    /// use provwasm_std::{ProvenanceQuerier, Marker, ProvenanceQuery};
    ///
    /// // Query a marker by denom.
    /// fn try_get_marker_by_denom(deps: Deps<ProvenanceQuery>, denom: String) -> StdResult<QueryResponse> {
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

    // Execute a scope query against the metadata module.
    fn query_scope(&self, params: MetadataQueryParams) -> StdResult<Scope> {
        let request = ProvenanceQuery {
            route: ProvenanceRoute::Metadata,
            params: ProvenanceQueryParams::Metadata(params),
            version: String::from(QUERY_DATAFMT_VERSION),
        };
        let res: Scope = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Get a scope by metadata ID (bech32 address string).
    ///
    /// ### Example
    /// ```rust
    /// // Imports required
    /// use provwasm_std::{ProvenanceQuerier, ProvenanceQuery, Scope};
    /// use cosmwasm_std::{Deps, QueryResponse, StdResult};
    ///
    /// // Query a scope by id.
    /// fn try_get_scope(deps: Deps<ProvenanceQuery>, scope_id: String) -> StdResult<QueryResponse> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let scope: Scope = querier.get_scope(scope_id)?;
    ///     // Do something with scope ...
    ///     todo!()
    /// }
    /// ```
    pub fn get_scope<S: Into<String>>(&self, scope_id: S) -> StdResult<Scope> {
        self.query_scope(MetadataQueryParams::GetScope {
            scope_id: validate_string(scope_id, "scope_id")?,
        })
    }

    // Execute a sessions query against the metadata module.
    fn query_sessions(&self, params: MetadataQueryParams) -> StdResult<Sessions> {
        let request = ProvenanceQuery {
            route: ProvenanceRoute::Metadata,
            params: ProvenanceQueryParams::Metadata(params),
            version: String::from(QUERY_DATAFMT_VERSION),
        };
        let res: Sessions = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Get all scope sessions.
    ///
    /// ### Example
    /// ```rust
    /// // Imports required
    /// use provwasm_std::{ProvenanceQuerier, ProvenanceQuery, Sessions};
    /// use cosmwasm_std::{Deps, QueryResponse, StdResult};
    ///
    /// // Query all sessions for a scope.
    /// fn try_get_sessions(deps: Deps<ProvenanceQuery>, scope_id: String) -> StdResult<QueryResponse> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let res: Sessions = querier.get_sessions(scope_id)?;
    ///     // Do something with res.sessions ...
    ///     todo!()
    /// }
    /// ```
    pub fn get_sessions<S: Into<String>>(&self, scope_id: S) -> StdResult<Sessions> {
        let scope_id = validate_string(scope_id, "scope_id")?;
        self.query_sessions(MetadataQueryParams::GetSessions { scope_id })
    }

    // Execute a record query against the metadata module.
    fn query_records(&self, params: MetadataQueryParams) -> StdResult<Records> {
        let request = ProvenanceQuery {
            route: ProvenanceRoute::Metadata,
            params: ProvenanceQueryParams::Metadata(params),
            version: String::from(QUERY_DATAFMT_VERSION),
        };
        let res: Records = self.querier.query(&request.into())?;
        Ok(res)
    }

    /// Get all scope records.
    ///
    /// ### Example
    /// ```rust
    /// // Imports required
    /// use provwasm_std::{ProvenanceQuerier, ProvenanceQuery, Records};
    /// use cosmwasm_std::{Deps, QueryResponse, StdResult};
    ///
    /// // Query all records for a scope.
    /// fn try_get_records(deps: Deps<ProvenanceQuery>, scope_id: String) -> StdResult<QueryResponse> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let res: Records = querier.get_records(scope_id)?;
    ///     // Do something with res.records ...
    ///     todo!()
    /// }
    /// ```
    pub fn get_records<S: Into<String>>(&self, scope_id: S) -> StdResult<Records> {
        let scope_id = validate_string(scope_id, "scope_id")?;
        let name: Option<String> = None;
        self.query_records(MetadataQueryParams::GetRecords { scope_id, name })
    }

    /// Get a scope record with the given name.
    ///
    /// ### Example
    /// ```rust
    /// // Imports required
    /// use provwasm_std::{ProvenanceQuerier, ProvenanceQuery, Record};
    /// use cosmwasm_std::{Deps, QueryResponse, StdResult};
    ///
    /// // Query a loan record for a scope.
    /// fn try_get_loan_record(deps: Deps<ProvenanceQuery>, scope_id: String) -> StdResult<QueryResponse> {
    ///     let querier = ProvenanceQuerier::new(&deps.querier);
    ///     let record: Record = querier.get_record_by_name(scope_id, "loan")?;
    ///     // Do something with record ...
    ///     todo!()
    /// }
    /// ```
    pub fn get_record_by_name<S: Into<String>, T: Into<String>>(
        &self,
        scope_id: S,
        name: T,
    ) -> StdResult<Record> {
        let scope_id = validate_string(scope_id, "scope_id")?;
        let name: String = validate_string(name, "name")?;
        let res = self.query_records(MetadataQueryParams::GetRecords {
            scope_id,
            name: Some(name.clone()),
        })?;
        if res.records.is_empty() {
            return Err(StdError::not_found(format!("record not found: {name}")));
        }
        Ok(res.records[0].clone())
    }
}
