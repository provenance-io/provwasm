# Provenance CosmWasm Bindings

This crate provides bindings to enable CosmWasm smart contracts to interact with custom provenance
blockchain modules.

| Crate          | Download                                                                                                                      | Docs                                                                                |
|----------------|-------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------|
| provwasm-std   | [![provwasm-std on crates.io](https://img.shields.io/crates/v/provwasm-std.svg)](https://crates.io/crates/provwasm-std)       | [![Docs](https://docs.rs/provwasm-std/badge.svg)](https://docs.rs/provwasm-std)     |
| provwasm-mocks | [![provwasm-mocks on crates.io](https://img.shields.io/crates/v/provwasm-mocks.svg)](https://crates.io/crates/provwasm-mocks) | [![Docs](https://docs.rs/provwasm-mocks/badge.svg)](https://docs.rs/provwasm-mocks) |

## Compatibility

The following table shows provwasm version compatibility for smart contract development and testing.

| provwasm | wasmd   | cosmos  | provenance        | module support                                                      |
|----------|---------|---------|-------------------|---------------------------------------------------------------------|
| v2.3.0   | v0.51.X | v0.50.X | v1.19.X           | all Provenance and most built-in third-party                        |
| v2.2.0   | v0.30.X | v0.46.X | v1.18.X           | attribute,exchange,hold,marker,metadata,msgfees,name,reward,trigger |
| v2.1.0   | v0.30.X | v0.46.X | v1.17.X           | attribute,exchange,hold,marker,metadata,msgfees,name,reward,trigger |
| v2.0.0   | v0.30.X | v0.46.X | v1.15.X           | attribute,marker,metadata,msgfees,name,reward                       |
| v1.2.0   | v0.30.X | v0.46.X | v1.15.X           | attribute,marker,metadata,msgfees,name                              |
| v1.1.2   | v0.29.X | v0.46.X | v1.13.X           | attribute,marker,metadata,msgfees,name                              |
| v1.1.1   | v0.29.X | v0.46.X | v1.13.X           | attribute,marker,metadata,msgfees,name                              |
| v1.1.0   | v0.26.X | v0.45.X | v1.10.X - v1.12.X | attribute,marker,metadata,msgfees,name                              |
| v1.0.0   | v0.26.X | v0.45.X | v1.8.X - v1.9.X   | attribute,marker,metadata,name                                      |

## Getting Started

Start with the [tutorial](docs/tutorial/01-overview.md) for a complete guide to developing smart
contracts for the Provenance Blockchain.

## IBC

Once the tutorial has been completed and you are ready for a more advanced topic, checkout the
[IBC Overview](contracts/ibc/README.md) to learn more about IBC Smart Contracts and a guided tour of a multi-contract
IBC project.

## Contents

In addition to the core CosmWasm functionality, the Provenance bindings include:

### Query Support

Since migrating to Stargate queries, only a subset of queries are responded to.
The [list](https://github.com/provenance-io/provenance/blob/7d6c507cab780bb6f0bdeef1e895c870cf4c7465/internal/provwasm/stargate_whitelist.go#L56)
is maintained
in [Provenance](https://github.com/provenance-io/provenance/).

__The following are known to have deserialization issues and will be `None` until upstream is fixed__ (
see [this issue](https://github.com/provenance-io/provwasm/issues/132)):

- `ContractSpecification.source`
- `InputSpecification.source`
- `Process.process_id`
- `SessionIdComponents.scope_identifier`
- `QualifyingAction.type`

### Message Encoding

_Provenance includes third-party protobuf definitions to maintain compatibility_

- [provenance](packages/provwasm-std/src/types/provenance)
- [capability](packages/provwasm-std/src/types/capability)
- [cosmos](packages/provwasm-std/src/types/cosmos)
- [cosmwasm](packages/provwasm-std/src/types/cosmwasm)
- [ibc](packages/provwasm-std/src/types/ibc)
- [tendermint](packages/provwasm-std/src/types/tendermint)

## Migration

Please see this [guide](./MIGRATION.md) for information regarding api changes and upgrading contracts