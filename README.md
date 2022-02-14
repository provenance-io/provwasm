# Provenance CosmWasm Bindings

This crate provides bindings to enable CosmWasm smart contracts to interact with custom provenance
blockchain modules.

| Crate          | Download | Docs |
| -------------- | -------- | ---- |
| provwasm-std   | [![provwasm-std on crates.io](https://img.shields.io/crates/v/provwasm-std.svg)](https://crates.io/crates/provwasm-std) |  [![Docs](https://docs.rs/provwasm-std/badge.svg)](https://docs.rs/provwasm-std) |
| provwasm-mocks | [![provwasm-mocks on crates.io](https://img.shields.io/crates/v/provwasm-mocks.svg)](https://crates.io/crates/provwasm-mocks) | [![Docs](https://docs.rs/provwasm-mocks/badge.svg)](https://docs.rs/provwasm-mocks) |

## Compatibility

The following table shows provwasm version compatibility for smart contract development and testing.

| provwasm      | wasmd          | cosmos  | provenance    | module support                 |
| ------------- | -------------- | ------- | ------------- | ------------------------------ |
| v1.0.1-beta   | v0.22.0        | v0.45.0 | v1.8.0+       | attribute,marker,metadata,name |
| v0.16.0       | v0.18.0        | v0.43.0 | v1.6.0+       | attribute,marker,metadata,name |
| v0.14.3       | v0.17.0        | v0.42.6 | v1.5.0+       | attribute,marker,metadata,name |
| v0.14.2       | v0.17.0        | v0.42.6 | v1.5.0+       | attribute,marker,metadata,name |
| v0.14.1       | v0.17.0        | v0.42.5 | v1.4.1+       | attribute,marker,name          |
| v0.14.0       | v0.16.0        | v0.42.4 | v1.3.0+       | attribute,marker,name          |
| v0.14.0-beta1 | v0.16.0-alpha1 | v0.42.3 | v0.2.1-v1.2.0 | attribute,marker,name          |
| v0.13         | v0.15          | v0.41   | v0.2.0        | attribute,marker,name          |

## Getting Started

Start with the [tutorial](docs/tutorial/01-overview.md) for a complete guide to developing smart
contracts for the Provenance Blockchain.

## Contents

In addition to the core CosmWasm functionality, the provenance bindings include

### Query Support

- Names
  - Resolve the address for a name.
  - Lookup all names bound to an address.
- Account Metadata
  - Get all attributes for an account.
  - Get attributes for an account by name.
- Markers
  - Get marker by denom.
  - Get marker by address.
- Metadata
  - Get scope by ID.
  - Get scope sessions.
  - Get scope records.
  - Get scope record by name.

### Message Encoding

- Names
  - Bind name
  - Delete name
- Account Metadata
  - Add attribute
  - Delete attributes
- Markers
  - Create
  - Grant access
  - Revoke access
  - Finalize
  - Activate
  - Cancel
  - Destroy
  - Mint supply
  - Burn supply
  - Withdraw coins
  - Transfer coins
- Metadata
  - Write Scope
