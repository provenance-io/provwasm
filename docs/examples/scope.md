# Module: Metadata

## Scopes

This document contains example code snippets for smart contract integration with the provenance
metadata module, specifically for querying scopes.

## Query

To query a scope by ID

```rust
use cosmwasm_std::{to_binary, Deps, QueryResponse, StdError};
use provwasm_std::{ProvenanceQuerier, Scope};

// Use a ProvenanceQuerier to get a scope by ID.
fn try_get_scope(deps: Deps, id: String) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let scope: Scope = querier.get_scope(id)?;
    to_binary(&scope)
}
```
