# Provenance CosmWasm Bindings

This crate provides bindings to enable CosmWasm smart contracts to interact with custom provenance
blockchain modules.

| Crate          | Download                                                                                                                      | Docs                                                                                |
|----------------|-------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------|
| provwasm-std   | [![provwasm-std on crates.io](https://img.shields.io/crates/v/provwasm-std.svg)](https://crates.io/crates/provwasm-std)       | [![Docs](https://docs.rs/provwasm-std/badge.svg)](https://docs.rs/provwasm-std)     |
| provwasm-mocks | [![provwasm-mocks on crates.io](https://img.shields.io/crates/v/provwasm-mocks.svg)](https://crates.io/crates/provwasm-mocks) | [![Docs](https://docs.rs/provwasm-mocks/badge.svg)](https://docs.rs/provwasm-mocks) |

## Compatibility

The following table shows provwasm version compatibility for smart contract development and testing.

| provwasm | wasmd   | cosmos  | provenance        | module support                                 |
|----------|---------|---------|-------------------|------------------------------------------------|
| v2.0.0   | v0.30.X | v0.46.X | v1.15.X           | attribute,marker,metadata,msgfees,name, reward |
| v1.2.0   | v0.30.X | v0.46.X | v1.15.X           | attribute,marker,metadata,msgfees,name         |
| v1.1.2   | v0.29.X | v0.46.X | v1.13.X           | attribute,marker,metadata,msgfees,name         |
| v1.1.1   | v0.29.X | v0.46.X | v1.13.X           | attribute,marker,metadata,msgfees,name         |
| v1.1.0   | v0.26.X | v0.45.X | v1.10.X - v1.12.X | attribute,marker,metadata,msgfees,name         |
| v1.0.0   | v0.26.X | v0.45.X | v1.8.X - v1.9.X   | attribute,marker,metadata,name                 |

## Getting Started

Start with the [tutorial](docs/tutorial/01-overview.md) for a complete guide to developing smart
contracts for the Provenance Blockchain.

## IBC

Once the tutorial has been completed and you are ready for a more advanced topic, checkout the
[IBC Overview](contracts/ibc/README.md) to learn more about IBC Smart Contracts and a guided tour of a multi-contract IBC project.

## Contents

In addition to the core CosmWasm functionality, the provenance bindings include

### Query Support

All queries are available for these Provenance modules:
- [Attribute](./packages/provwasm-std/src/types/provenance/attribute)
  - The following has deserialization issues and will be `None` until fixed:
    - `ContractSpecification.source`
    - `InputSpecification.source`
    - `Process.process_id`
    - `SessionIdComponents.scope_identifier`
- [Marker](./packages/provwasm-std/src/types/provenance/marker)
- [Metadata](./packages/provwasm-std/src/types/provenance/metadata)
- [MsgFees](./packages/provwasm-std/src/types/provenance/msgfees)
- [Name](./packages/provwasm-std/src/types/provenance/name)
- [Reward](./packages/provwasm-std/src/types/provenance/reward)
  - The following has deserialization issues and will be `None` until fixed:
    - `QualifyingAction.type`

### Message Encoding

All messages are available for these Provenance modules:
- [Attribute](./packages/provwasm-std/src/types/provenance/attribute)
- [Marker](./packages/provwasm-std/src/types/provenance/marker)
- [Metadata](./packages/provwasm-std/src/types/provenance/metadata)
- [MsgFees](./packages/provwasm-std/src/types/provenance/msgfees)
- [Name](./packages/provwasm-std/src/types/provenance/name)
- [Reward](./packages/provwasm-std/src/types/provenance/reward)

## Migration
Please see this [guide](./MIGRATION.md) for information regarding api changes and upgrading contracts