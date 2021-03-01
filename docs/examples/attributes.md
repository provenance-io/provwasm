# Module: Attribute

## Account Attributes

This document contains example code snippets for smart contract integration with the provenance
attribute module.

These bindings allow for Rust structs to be stored under an account (identified by address), with
type of `json`, as long as they derive the `Serialize` and `Deserialize` traits from the `serde`
library.

For example

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Label text with a unix timestamp.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Label {
    pub text: String,
    pub timestamp: u64,
}

// A collection of labels.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Labels {
    pub labels: Vec<Label>,
}
```

## Add Attribute

To add a `Label` attribute to an account

```rust
use cosmwasm_std::{Env, HandleResponse};
use provwasm_std::{add_json_attribute, ProvenanceMsg};
use crate::error::ContractError;
use create::msg::Label;

// Add a label attribute.
fn handle_add_label(
    env: Env,
    address: HumanAddr,
    text: String,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let attr_name = String::from("label.my-contract.sc.pb"); // NOTE: Name must resolve to contract address.
    let timestamp = env.block.time;
    let label = Label { text, timestamp };
    let msg = add_json_attribute(&address, &attr_name, &label)?;
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Query Attributes

To query for `Label` attributes under an account (for use in a `query` handler)

```rust
use cosmwasm_std::{to_binary, Deps, HumanAddr, QueryResponse, StdError};
use provwasm_std::ProvenanceQuerier;
use crate::msg::{Label, Labels};

// Query all label attributes for an account.
pub fn query_labels(deps: Deps, address: HumanAddr) -> Result<QueryResponse, StdError> {
    let attr_name = String::from("label.my-contract.sc.pb");
    let querier = ProvenanceQuerier::new(&deps.querier);
    let labels: Vec<Label> = querier.get_json_attributes(&address, &attr_name)?;
    to_binary(&Labels { labels })
}
```

## Delete Attributes

```rust
use cosmwasm_std::HandleResponse;
use provwasm_std::{delete_attributes, ProvenanceMsg};
use crate::error::ContractError;

// Delete all label attributes.
fn handle_delete_labels(
    address: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let attr_name = String::from("label.my-contract.sc.pb"); // NOTE: Name must resolve to contract address.
    let msg = delete_attributes(&address, &attr_name);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```
