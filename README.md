# Provenance CosmWasm Bindings

This crate provides bindings to enable CosmWasm smart contracts to interact with custom provenance
blockchain modules.

| Crate          | Download                                                                                                                      | Docs                                                                                |
|----------------|-------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------|
| provwasm-std   | [![provwasm-std on crates.io](https://img.shields.io/crates/v/provwasm-std.svg)](https://crates.io/crates/provwasm-std)       | [![Docs](https://docs.rs/provwasm-std/badge.svg)](https://docs.rs/provwasm-std)     |
| provwasm-mocks | [![provwasm-mocks on crates.io](https://img.shields.io/crates/v/provwasm-mocks.svg)](https://crates.io/crates/provwasm-mocks) | [![Docs](https://docs.rs/provwasm-mocks/badge.svg)](https://docs.rs/provwasm-mocks) |

## Compatibility

The following table shows provwasm version compatibility for smart contract development and testing.

| provwasm | wasmd   | cosmos  | provenance        | module support                         |
|----------|---------|---------|-------------------|----------------------------------------|
| v1.2.0   | v0.30.X | v0.46.X | v1.15.X           | attribute,marker,metadata,msgfees,name |
| v1.1.2   | v0.29.X | v0.46.X | v1.13.X           | attribute,marker,metadata,msgfees,name |
| v1.1.1   | v0.29.X | v0.46.X | v1.13.X           | attribute,marker,metadata,msgfees,name |
| v1.1.0   | v0.26.X | v0.45.X | v1.10.X - v1.12.X | attribute,marker,metadata,msgfees,name |
| v1.0.0   | v0.26.X | v0.45.X | v1.8.X - v1.9.X   | attribute,marker,metadata,name         |

## Getting Started

Start with the [tutorial](docs/tutorial/01-overview.md) for a complete guide to developing smart
contracts for the Provenance Blockchain.

## IBC

Once the tutorial has been completed and you are ready for a more advanced topic, checkout the
[IBC Overview](contracts/ibc/README.md) to learn more about IBC Smart Contracts and a guided tour of a multi-contract IBC project.

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
  - Delete all attributes
  - Delete distinct attribute
  - Update attribute
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
- MsgFees
  - Assess custom fee
