# Provenance CosmWasm Bindings

This crate provides bindings to enable CosmWasm smart contracts to interact with custom provenance
blockchain modules.

| Crate          | Download | Docs |
| -------------- | -------- | ---- |
| provwasm-std   | [![provwasm-std on crates.io](https://img.shields.io/crates/v/provwasm-std.svg)](https://crates.io/crates/provwasm-std) |  [![Docs](https://docs.rs/provwasm-std/badge.svg)](https://docs.rs/provwasm-std) |
| provwasm-mocks | [![provwasm-mocks on crates.io](https://img.shields.io/crates/v/provwasm-mocks.svg)](https://crates.io/crates/provwasm-mocks) | [![Docs](https://docs.rs/provwasm-mocks/badge.svg)](https://docs.rs/provwasm-mocks) |

## Compatibility

The following table shows provwasm version compatibility and network availability for smart
contract development and testing.

| provwasm | wasmd   | cosmos  | provenance | localnet | testnet | mainnet | module support        |
| -------- | ------- | ------- | ---------- | -------- | ------- | ------- | --------------------- |
| v0.13.x  | v0.15.0 | v0.41.0 | v0.1.x     | yes      | yes     | N/A     | attribute,marker,name |

## Getting Started

Start with the [tutorial](docs/tutorial/01-overview.md) for a complete guide to developing smart
contracts for the Provenance Blockchain.

## Example Usage

See the [examples](docs/examples/README.md) for usage.

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
