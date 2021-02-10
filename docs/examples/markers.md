# Module: Marker

This document contains example code snippets for smart contract integration with the provenance
marker module.

## Imports

Imports use for marker examples

```rust
use cosmwasm_std::{to_binary, Coin, Deps, HandleResponse, HumanAddr, QueryResponse, StdError};
use provwasm_std::{
    activate_marker, burn_marker_supply, cancel_marker, create_marker, destroy_marker,
    finalize_marker, grant_marker_access, grant_marker_access_all, grant_marker_access_asset,
    grant_marker_access_supply, mint_marker_supply, withdraw_marker_coins, Marker, ProvenanceMsg,
    ProvenanceQuerier,
};
use create::error::ContractError;
```

## Query

To query a marker by denomination

```rust
// Query a marker by denom.
fn try_get_marker_by_denom(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker: Marker = querier.get_marker_by_denom(denom)?;
    to_binary(&marker)
}
```

To query a marker by address

```rust
// Query a marker by address.
fn try_get_marker_by_address(deps: Deps, address: HumanAddr) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker: Marker = querier.get_marker_by_address(address)?;
    to_binary(&marker)
}
```

## Create

To create a new proposed marker

```rust
// Create and dispatch a message that will create a new proposed marker.
fn try_create_marker(coin: Coin) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = create_marker(coin);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Grant

To add all permissions (burn, delete, deposit, grant, mint, withdraw) to an account.

```rust
// Create and dispatch a message that will grant all permissions to a marker for an address.
fn try_grant_marker_access(
    denom: String,
    address: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = grant_marker_access_all(denom, address);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

To add supply permissions (mint, burn) to an account.

```rust
// Create and dispatch a message that will grant supply permissions to a marker for an address.
fn try_grant_marker_access(
    denom: String,
    address: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = grant_marker_access_supply(denom, address);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

To add asset permissions (deposit, withdraw) to an account.

```rust
// Create and dispatch a message that will grant asset permissions to a marker for an address.
fn try_grant_marker_access(
    denom: String,
    address: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = grant_marker_access_asset(denom, address);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

To add custom permissions to an account.

```rust
// Create and dispatch a message that will grant custom permissions to a marker for an address.
fn try_grant_marker_access(
    denom: String,
    address: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let permissions = vec![MarkerPermission::Deposit, MarkerPermission::Mint];
    let msg = grant_marker_access(denom, address, permissions);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Revoke

To revoke all permissions from an account.

```rust
// Create and dispatch a message that will revoke all permissions from a marker for an address.
fn try_revoke_marker_access(
    denom: String,
    address: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = revoke_marker_access(denom, address);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Finalize

To set marker status to finalized

```rust
// Create and dispatch a message that will finalize a proposed marker.
fn try_finalize_marker(denom: String) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = finalize_marker(denom);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Activate

To set marker status to active (mints supply)

```rust
// Create and dispatch a message that will activate a finalized marker.
fn try_activate_marker(denom: String) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = activate_marker(denom);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Mint

To mint marker supply

```rust
// Create and dispatch a message that will mint marker supply.
fn try_mint_marker(coin: Coin) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = mint_marker_supply(coin);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Burn

To burn marker supply

```rust
// Create and dispatch a message that will burn marker supply.
fn try_burn_marker(coin: Coin) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = burn_marker_supply(coin);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Cancel

To cancel a marker

```rust
// Create and dispatch a message that will cancel a marker.
fn try_cancel_marker(denom: String) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = cancel_marker(denom);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Destroy

To destroy a marker

```rust
// Create and dispatch a message that will destroy a marker.
fn try_destroy_marker(denom: String) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = destroy_marker(denom);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Transfer

To transfer marker coins between addresses

```rust

// Create and dispatch a message that will transfer marker coins from one account to another .
fn try_transfer_marker_coins(
    coin: Coin,
    to: HumanAddr,
    from: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = transfer_marker_coins(coin, to, from);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```

## Withdraw

To withdraw marker coins to a recipient address

```rust
// Create and dispatch a message that will withdraw coins from a marker.
fn try_withdraw_marker_coins(
    coin: Coin,
    recipient: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = withdraw_marker_coins(coin, recipient);
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![],
        data: None,
    })
}
```
