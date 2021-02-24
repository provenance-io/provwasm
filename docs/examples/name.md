# Module: Name

This document contains example code snippets for smart contract integration with the provenance
name module.

## Bind Name

To bind a name to an address for use in a smart contract `handle` function

```rust
use cosmwasm_std::{HandleResponse, HumanAddr};
use provwasm_std::{bind_name, ProvenanceMsg};
use crate::error::ContractError;

// Bind a name to an address
fn handle_bind_name(
    name: String,
    address: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = bind_name(&name, &address);
    Ok(HandleResponse {
        messages: vec![msg], // Will be dispatched to name module handler
        attributes: vec![],
        data: None,
    })
}
```

## Unbind Name

To unbind a name from an address for use in a smart contract `handle` function

```rust
use cosmwasm_std::{HandleResponse, HumanAddr};
use provwasm_std::{bind_name, ProvenanceMsg};
use crate::error::ContractError;

// Unbind a name from an address
fn handle_bind_name(name: String) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = unbind_name(&name);
    Ok(HandleResponse {
        messages: vec![msg], // Will be dispatched to name module handler
        attributes: vec![],
        data: None,
    })
}
```

## Resolve Address

To resolve the address for a name in a smart contract `query` function

```rust
use cosmwasm_std::{to_binary, Deps, QueryResponse, StdError};
use provwasm_std::{Name, ProvenanceQuerier};

// Use a ProvenanceQuerier to resolve the address for a name.
fn query_resolve_name(deps: Deps, name: String) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let name: Name = querier.resolve_name(&name)?;
    // Do something with name...
    to_binary(&name)
}
```

## Lookup Names

To lookup all addresses bound to a name in a smart contract `query` function

```rust
use cosmwasm_std::{to_binary, Deps, HumanAddr, QueryResponse, StdError};
use provwasm_std::{Names, ProvenanceQuerier};

// Use a ProvenanceQuerier to lookup all names bound to an address.
fn query_lookup_names(deps: Deps, address: HumanAddr) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let names: Names = querier.lookup_names(&address)?;
    // Do something with names...
    to_binary(&names)
}
```
